use leptos::prelude::*;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use canonrs_ui::blocks::{Card, AlertBlock, AlertVariant, StatCard, CalloutBlock, CalloutType, EmptyState, ToolbarBlock, PageHeader, Breadcrumb, BreadcrumbItem, ButtonGroupBlock, FormBlock, FieldBlock, FormActionsBlock, DataTableBlock, List, ListItem, Table, CodeBlockBlock};
use crate::ui::layout_builder::types::{Node, NodeKind, DragContext, CanvasMode};
use rs_canonrs::application::Command;
use super::drop_zone::DropZone;
use super::region_preview::RegionPreview;

#[component]
pub fn BlockPreview(
    node: Node,
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<DragContext>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<super::drop_zone::DragVisualState>,
) -> impl IntoView {
    let node_id = node.id;
    let is_builder = move || canvas_mode.get() == CanvasMode::Builder;
    let is_selected = move || selected_id.get() == Some(node_id);
    let is_container = node.is_container();
    let is_dragging_this = move || drag_ctx.get().node_id == Some(node_id);

    let (block_id_owned, block_label_owned, block_icon_owned): (String, String, String) = match &node.kind {
        NodeKind::Block { def } => (def.id.to_string(), def.label.to_string(), def.icon.to_string()),
        NodeKind::Slot { name } => (name.clone(), name.clone(), "▤".to_string()),
        NodeKind::Region { region_id, label, .. } => (region_id.to_string(), label.to_string(), "▤".to_string()),
        NodeKind::Component { def } => (def.id.to_string(), def.label.to_string(), def.icon.to_string()),
        NodeKind::Text { variant, .. } => (variant.tag().to_string(), variant.label().to_string(), "T".to_string()),
        NodeKind::Layout { id, label } => (id.clone(), label.clone(), "📐".to_string()),
    };
    let block_id = block_id_owned.as_str();
    let block_label = block_label_owned.as_str();
    let block_icon = block_icon_owned.as_str();

    // Regiões do bloco — calculadas UMA VEZ no mount, estáticas
    let regions: Vec<(&'static str, &'static str)> = match &node.kind {
        NodeKind::Block { def } => def.regions.iter().map(|r| (r.id, r.label)).collect(),
        _ => vec![],
    };
    let has_regions = !regions.is_empty();

    // IDs dos nós de região — lidos uma vez do estado atual da tree
    // Se não existirem ainda, serão criados atomicamente
    let block_id_for_regions = block_id_owned.clone();
    let region_nodes_memo = Memo::new(move |_| {
        if !has_regions { return vec![]; }
        let t = tree.get();
        leptos::logging::log!("[region_memo] tree={} node_id={} regions={}", t.len(), node_id, regions.len());
        for n in t.iter().filter(|n| n.parent_id == Some(node_id)) {
            leptos::logging::log!("[region_memo] child: {:?}", n.kind);
        }
        regions.iter().filter_map(|(rid, rlabel)| {
            t.iter().find(|n|
                n.parent_id == Some(node_id) &&
                matches!(&n.kind, crate::ui::layout_builder::types::NodeKind::Region { region_id, .. } if region_id == *rid)
            ).cloned()
        }).collect::<Vec<_>>()
    });


    let bd1 = match node.kind.clone() { NodeKind::Block { def } => Some(def), _ => None };
    let bd2 = bd1.clone();
    let cd1 = match node.kind.clone() { NodeKind::Component { def } => Some(def), _ => None };
    let cd2 = cd1.clone();

    use crate::ui::layout_builder::types::TextVariant;
    let text_variant = match &node.kind {
        NodeKind::Text { variant, .. } => Some(*variant),
        NodeKind::Block { def } => match def.id {
            "text-h1"      => Some(TextVariant::Heading1),
            "text-h2"      => Some(TextVariant::Heading2),
            "text-h3"      => Some(TextVariant::Heading3),
            "text-p"       => Some(TextVariant::Paragraph),
            "text-label"   => Some(TextVariant::Label),
            "text-caption" => Some(TextVariant::Caption),
            _ => None,
        },
        _ => None,
    };
    let init_text = match &node.kind {
        NodeKind::Text { content, .. } => content.clone(),
        _ => String::new(),
    };
    let text_signal: RwSignal<String> = RwSignal::new(init_text);

    let remove = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        ev.prevent_default();
        drag_ctx.set(DragContext::empty());
        if selected_id.get_untracked() == Some(node_id) {
            selected_id.set(None);
        }
        engine.update(|e| { let _ = e.execute(Command::Remove { node_id }); });
        let flat = engine.get_untracked().sync_flat();
        tree.set(flat);
    };

    let on_click = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        selected_id.set(Some(node_id));
    };

    let pending_drag: RwSignal<Option<(f64, f64)>> = RwSignal::new(None);
    let is_region = matches!(node.kind, NodeKind::Region { .. });
    let is_draggable = !is_region && (bd1.is_some() || cd1.is_some());

    let on_pointerdown = move |ev: leptos::ev::PointerEvent| {
        ev.stop_propagation();
        if is_draggable {
            ev.prevent_default();
            pending_drag.set(Some((ev.client_x() as f64, ev.client_y() as f64)));
        }
    };

    let on_pointermove_block = move |ev: leptos::ev::PointerEvent| {
        if let Some((start_x, start_y)) = pending_drag.get_untracked() {
            let dx = ev.client_x() as f64 - start_x;
            let dy = ev.client_y() as f64 - start_y;
            if (dx * dx + dy * dy).sqrt() > 5.0 {
                let (bd, cd) = if let Some(def) = bd2.clone() {
                    (Some(def), None)
                } else if let Some(def) = cd2.clone() {
                    (None, Some(def))
                } else { (None, None) };
                drag_ctx.set(DragContext { node_id: Some(node_id), block_def: bd, component_def: cd, layout_def: None });
                pending_drag.set(None);
            }
        }
    };

    let on_pointerup_block = move |_: leptos::ev::PointerEvent| {
        pending_drag.set(None);
    };

    let inner = if let Some(variant) = text_variant {
        let placeholder = variant.placeholder();
        let font_style = match variant {
            TextVariant::Heading1 => "font-size: 1.5rem; font-weight: 700; line-height: 1.2;",
            TextVariant::Heading2 => "font-size: 1.25rem; font-weight: 600; line-height: 1.3;",
            TextVariant::Heading3 => "font-size: 1.1rem; font-weight: 600; line-height: 1.4;",
            TextVariant::Paragraph => "font-size: 0.9rem; line-height: 1.6;",
            TextVariant::Label => "font-size: 0.8rem; font-weight: 500;",
            TextVariant::Caption => "font-size: 0.75rem; color: var(--theme-surface-fg-muted);",
        };
        let style_str = format!("{} min-width: 40px; outline: none; cursor: text; padding: 0.1rem 0.25rem; border-radius: 2px;", font_style);
        view! {
            <div
                data-text-block=""
                contenteditable=move || if is_builder() { "true" } else { "false" }
                attr:data-placeholder=placeholder
                style=style_str
                on:input=move |ev| {
                    use leptos::wasm_bindgen::JsCast;
                    if let Some(el) = ev.target()
                        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        let val = el.inner_text();
                        text_signal.set(val.clone());
                        // UpdateProps no engine para persistencia
                        engine.update(|e| {
                            let _ = e.execute(rs_canonrs::application::Command::UpdateProps {
                                node_id,
                                key: "content".to_string(),
                                value: serde_json::Value::String(val.clone()),
                            });
                        });
                        // Atualizar projecao flat local (text content nao requer rebuild completo)
                        tree.update(|t| {
                            if let Some(n) = t.iter_mut().find(|n| n.id == node_id) {
                                n.kind = NodeKind::Text { content: val, variant };
                            }
                        });
                    }
                }
                on:click=move |ev| ev.stop_propagation()
                on:pointerdown=move |ev| ev.stop_propagation()
            >
                {move || { let t = text_signal.get(); if t.is_empty() { String::new() } else { t } }}
            </div>
        }.into_any()
    } else {
        match block_id {
            "card" => {
                let region_ids: Vec<(String, uuid::Uuid)> = region_nodes_memo.get()
                    .iter()
                    .filter_map(|n| match &n.kind {
                        crate::ui::layout_builder::types::NodeKind::Region { region_id, .. } => Some((region_id.clone(), n.id)),
                        _ => None,
                    })
                    .collect();
                let get_zone = move |rid: &str| region_ids.iter().find(|(r,_)| r == rid).map(|(_,id)| *id);
                let header_id = get_zone("header");
                let content_id = get_zone("content");
                let footer_id = get_zone("footer");
                view! {
                    <div data-block="card" data-block-version="1" data-variant="default">
                        <div data-block-region="header">
                            {header_id.map(|pid| view! {
                                <DropZone parent_id=pid engine=engine tree=tree drag_ctx=drag_ctx
                                    selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual
                                    slot_label="header".to_string() />
                            })}
                        </div>
                        <div data-block-region="content">
                            {content_id.map(|pid| view! {
                                <DropZone parent_id=pid engine=engine tree=tree drag_ctx=drag_ctx
                                    selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual
                                    slot_label="content".to_string() />
                            })}
                        </div>
                        <div data-block-region="footer">
                            {footer_id.map(|pid| view! {
                                <DropZone parent_id=pid engine=engine tree=tree drag_ctx=drag_ctx
                                    selected_id=selected_id canvas_mode=canvas_mode drag_visual=drag_visual
                                    slot_label="footer".to_string() />
                            })}
                        </div>
                    </div>
                }.into_any()
            },
            "alert" => view! {
                <AlertBlock variant=AlertVariant::Info>"Alert message"</AlertBlock>
            }.into_any(),
            "stat-card" => view! {
                <StatCard label="Metric".to_string() value="1,234".to_string() change="↑ 12%".to_string() />
            }.into_any(),
            "callout" => view! {
                <CalloutBlock variant=CalloutType::Info title="Callout".to_string()>
                    "Callout message"
                </CalloutBlock>
            }.into_any(),
            "empty-state" => view! {
                <EmptyState title="No data yet".to_string() description="Drop content here".to_string() />
            }.into_any(),
            "toolbar" => view! {
                <ToolbarBlock>
                    <span style="font-size:0.8rem;color:var(--theme-surface-fg-muted);">"Toolbar"</span>
                </ToolbarBlock>
            }.into_any(),
            "page-header" => view! {
                <PageHeader title="Page Title".to_string() subtitle="Page description".to_string() />
            }.into_any(),
            "breadcrumb" => view! {
                <Breadcrumb items=vec![
                    BreadcrumbItem { label: "Home".to_string(), href: Some("#".to_string()) },
                    BreadcrumbItem { label: "Page".to_string(), href: Some("#".to_string()) },
                    BreadcrumbItem { label: "Current".to_string(), href: None },
                ] />
            }.into_any(),
            "button-group" => view! {
                <ButtonGroupBlock>
                    <button class="canon-btn">"Action 1"</button>
                    <button class="canon-btn">"Action 2"</button>
                </ButtonGroupBlock>
            }.into_any(),
            "form" => view! {
                <FormBlock>
                    <div style="font-size:0.8rem;color:var(--theme-surface-fg-muted);padding:0.5rem;">"Drop fields here"</div>
                </FormBlock>
            }.into_any(),
            "field" => view! {
                <FieldBlock label="Field Label".to_string()>
                    <input type="text" placeholder="Value..." style="width:100%;padding:0.4rem 0.6rem;border:1px solid var(--theme-surface-border);border-radius:var(--radius-sm);background:var(--theme-surface-bg);color:var(--theme-surface-fg);font-size:0.85rem;" />
                </FieldBlock>
            }.into_any(),
            "form-actions" => view! {
                <FormActionsBlock>
                    <button class="canon-btn canon-btn--ghost">"Cancel"</button>
                    <button class="canon-btn canon-btn--primary">"Submit"</button>
                </FormActionsBlock>
            }.into_any(),
            "data-table" | "table" => view! {
                <DataTableBlock>
                    <div style="font-size:0.75rem;color:var(--theme-surface-fg-muted);padding:0.5rem;">"Table data"</div>
                </DataTableBlock>
            }.into_any(),
            "list" => view! {
                <List>
                    <ListItem>"Item 1"</ListItem>
                    <ListItem>"Item 2"</ListItem>
                    <ListItem>"Item 3"</ListItem>
                </List>
            }.into_any(),
            "code-block" => view! {
                <CodeBlockBlock language="rust".to_string() code={"fn main() { println!(\"Hello!\"); }"} />
            }.into_any(),
            _ => view! {
                <div data-block-inner="">
                    <span>{block_icon}</span>
                    <span>{block_label}</span>
                </div>
            }.into_any(),
        }
    };

    view! {
        <div
            attr:data-block-preview=move || if is_builder() { Some("") } else { None }
            attr:data-dragging=move || if is_builder() && is_dragging_this() { Some("true") } else { None }
            attr:data-selected=move || if is_builder() && is_selected() { Some("true") } else { None }
            style="position:relative;"
            on:click=on_click
            on:pointerdown=move |ev| { if !is_region { on_pointerdown(ev); } else { ev.stop_propagation(); } }
            on:pointermove=move |ev| { if !is_region { on_pointermove_block(ev); } }
            on:pointerup=move |ev| { if !is_region { on_pointerup_block(ev); } }
        >
            {inner}

            // Regiões estáticas — IDs capturados no mount
            {move || if !has_regions {
                region_nodes_memo.get().into_iter().map(|n| view! {
                    <RegionPreview node=n engine=engine tree=tree
                        drag_ctx=drag_ctx
                        selected_id=selected_id
                        canvas_mode=canvas_mode
                        drag_visual=drag_visual
                    />
                }.into_any()).collect::<Vec<_>>()
            } else { vec![] }}

            {move || if is_container && !has_regions {
                Some(view! {
                    <div
                        data-block-children=""
                        attr:data-mode=move || if is_builder() { "builder" } else { "preview" }
                        attr:data-drag-active=move || if drag_ctx.get().is_dragging() { "true" } else { "false" }
                        style=move || if drag_ctx.get().is_dragging() && is_builder() {
                            "margin-top: 0.25rem; border: 2px solid var(--builder-dropzone-active-border); border-radius: var(--builder-block-radius); background: var(--builder-dropzone-hover-bg); padding: 0.25rem;"
                        } else if is_builder() { "margin-top: 0.25rem;" } else { "" }
                        on:pointermove=move |ev| ev.stop_propagation()
                    >
                        <DropZone parent_id=node_id engine=engine tree=tree
                            drag_ctx=drag_ctx
                            selected_id=selected_id
                            canvas_mode=canvas_mode
                            drag_visual=drag_visual
                            slot_label="children".to_string()
                        />
                    </div>
                }.into_any())
            } else { None }}

            {move || if is_builder() {
                Some(view! {
                    <button
                        data-block-remove=""
                        on:click=remove
                        on:pointerdown=move |ev| ev.stop_propagation()
                    >
                        "×"
                    </button>
                })
            } else { None }}
        </div>
    }
}
