use leptos::prelude::*;
use crate::ui::layout_builder::state::builder_engine::BuilderEngine;
use canonrs_ui::blocks::{Card, AlertBlock, AlertVariant};
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
    let region_node_ids: Vec<uuid::Uuid> = if has_regions {
        let t = tree.get_untracked();
        let existing: Vec<uuid::Uuid> = regions.iter().filter_map(|(rid, _)| {
            t.iter().find(|n|
                n.parent_id == Some(node_id) &&
                matches!(&n.kind, NodeKind::Region { region_id, .. } if *region_id == *rid)
            ).map(|n| n.id)
        }).collect();

        if existing.len() == regions.len() {
            existing
        } else {
            // Cria regiões faltantes atomicamente
            if let Some(def) = crate::ui::layout_builder::domain::blocks::get_block(block_id).cloned() {
                let new_nodes: Vec<Node> = def.regions.iter().enumerate()
                    .filter(|(_, r)| !t.iter().any(|n|
                        n.parent_id == Some(node_id) &&
                        matches!(&n.kind, NodeKind::Region { region_id, .. } if *region_id == r.id)
                    ))
                    .map(|(i, r)| Node::region(block_id, r, node_id))
                    .collect();
                let new_ids: Vec<uuid::Uuid> = new_nodes.iter().map(|n| n.id).collect();
                // Só cria regiões se o nó pai ainda existe na tree
                // regiões criadas apenas no drop_handler
                // Relê da tree após criação
                let t2 = tree.get_untracked();
                regions.iter().filter_map(|(rid, _)| {
                    t2.iter().find(|n|
                        n.parent_id == Some(node_id) &&
                        matches!(&n.kind, NodeKind::Region { region_id, .. } if *region_id == *rid)
                    ).map(|n| n.id)
                }).collect()
            } else { vec![] }
        }
    } else { vec![] };

    // Nós de região estáticos para o view
    let region_nodes: Vec<Node> = {
        let t = tree.get_untracked();
        region_node_ids.iter().filter_map(|id| t.iter().find(|n| n.id == *id).cloned()).collect()
    };

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
                drag_ctx.set(DragContext { node_id: Some(node_id), block_def: bd, component_def: cd });
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
            "card" => view! {
                <Card>
                    <div style="padding: 0.5rem; font-size: 0.8rem; color: var(--theme-surface-fg-muted);">"Card content"</div>
                </Card>
            }.into_any(),
            "alert" => view! {
                <AlertBlock variant=AlertVariant::Info>"Alert message"</AlertBlock>
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
            data-block-preview=""
            attr:data-dragging=move || if is_dragging_this() { "true" } else { "false" }
            attr:data-selected=move || if is_selected() { "true" } else { "false" }
            attr:data-mode=move || if is_builder() { "builder" } else { "preview" }
            on:click=on_click
            on:pointerdown=move |ev| { if !is_region { on_pointerdown(ev); } else { ev.stop_propagation(); } }
            on:pointermove=move |ev| { if !is_region { on_pointermove_block(ev); } }
            on:pointerup=move |ev| { if !is_region { on_pointerup_block(ev); } }
        >
            {inner}

            // Regiões estáticas — IDs capturados no mount
            {region_nodes.into_iter().map(|n| view! {
                <RegionPreview node=n engine=engine tree=tree
                    drag_ctx=drag_ctx
                    selected_id=selected_id
                    canvas_mode=canvas_mode
                    drag_visual=drag_visual
                />
            }.into_any()).collect_view()}

            {move || if is_container && !has_regions {
                Some(view! {
                    <div
                        data-block-children=""
                        attr:data-mode=move || if is_builder() { "builder" } else { "preview" }
                        attr:data-drag-active=move || if drag_ctx.get().is_dragging() { "true" } else { "false" }
                        style=move || if drag_ctx.get().is_dragging() && is_builder() {
                            "margin-top: 0.25rem; border: 2px solid var(--builder-dropzone-active-border); border-radius: var(--builder-block-radius); background: var(--builder-dropzone-hover-bg); padding: 0.25rem;"
                        } else { "margin-top: 0.25rem;" }
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
