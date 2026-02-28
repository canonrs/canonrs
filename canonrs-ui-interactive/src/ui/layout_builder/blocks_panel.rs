use leptos::prelude::*;
use super::types::{Node, NodeKind, DragContext, ActiveLayout};
use super::state::builder_engine::BuilderEngine;
use canonrs_ui::ui::tabs::{Tabs, TabsList, TabsTrigger, TabsContent};

#[component]
pub fn BlocksPanel(
    tree: RwSignal<Vec<Node>>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    drag_ctx: RwSignal<DragContext>,
    active_layout: RwSignal<Option<ActiveLayout>>,
    engine: RwSignal<BuilderEngine>,
) -> impl IntoView {

    // Etapa atual do fluxo
    // 0 = Layouts, 1 = Blocks, 2 = UI
    let step: RwSignal<u8> = RwSignal::new(0);

    // Tem layout dropado? (tree tem pelo menos 1 node que não é slot)
    let has_layout = Memo::new(move |_| {
        tree.get().iter().any(|n| matches!(&n.kind, NodeKind::Layout { .. }))
    });

    // Tem pelo menos 1 block colocado dentro de layout?
    let has_blocks = Memo::new(move |_| {
        tree.get().iter().any(|n| !matches!(&n.kind, NodeKind::Slot { .. } | NodeKind::Region { .. } | NodeKind::Layout { .. }))
    });

    // Avança automaticamente de step quando condição atendida
    // Avança automaticamente quando layout é dropado pela primeira vez
    Effect::new(move |prev: Option<bool>| {
        let layout_just_added = has_layout.get();
        if layout_just_added && prev == Some(false) {
            step.set(1);
        }
        layout_just_added
    });
    Effect::new(move |prev: Option<bool>| {
        let blocks_just_added = has_blocks.get();
        if blocks_just_added && prev == Some(false) && step.get_untracked() == 1 {
            step.set(2);
        }
        blocks_just_added
    });

    let sidebar_context = Memo::new(move |_| {
        let sel_id = selected_id.get();
        sel_id.and_then(|id| tree.get_untracked().into_iter().find(|n| n.id == id))
            .and_then(|n| match &n.kind {
                NodeKind::Region { block_id, region_id, label } =>
                    Some((block_id.to_string(), region_id.to_string(), label.to_string())),
                _ => None,
            })
    });

    let blocks_list = Memo::new(move |_| {
        let ctx = sidebar_context.get();
        super::domain::blocks::BLOCK_REGISTRY.values().cloned().collect::<Vec<_>>().into_iter().filter(|b| {
            if let Some((ref bid, ref rid, _)) = ctx {
                super::domain::blocks::get_block(bid.as_str())
                    .and_then(|blk| blk.regions.iter().find(|r| r.id == rid.as_str()))
                    .map(|r| r.accepts_block(b.id, b.category))
                    .unwrap_or(true)
            } else { true }
        }).collect::<Vec<_>>()
    });

    let components_list = Memo::new(move |_| {
        let ctx = sidebar_context.get();
        super::domain::blocks::COMPONENT_REGISTRY.values().cloned().collect::<Vec<_>>().into_iter().filter(|c| {
            if let Some((ref bid, ref rid, _)) = ctx {
                super::domain::blocks::get_block(bid.as_str())
                    .and_then(|blk| blk.regions.iter().find(|r| r.id == rid.as_str()))
                    .map(|r| r.accepts_block(c.id, c.category))
                    .unwrap_or(true)
            } else { true }
        }).collect::<Vec<_>>()
    });

    let tab_style = move |idx: u8, enabled: bool| {
        let active = step.get() == idx;
        format!(
            "flex:1;padding:0.3rem 0.1rem;font-size:0.7rem;font-weight:600;border:none;border-bottom:2px solid {};background:transparent;color:{};cursor:{};transition:all 0.15s;",
            if active { "var(--theme-action-primary-bg)" } else { "transparent" },
            if !enabled { "var(--theme-surface-fg-muted)" } else if active { "var(--theme-action-primary-bg)" } else { "var(--theme-surface-fg)" },
            if enabled { "pointer" } else { "not-allowed" },
        )
    };

    let draggable_item = |icon: &'static str, label: &'static str, extra_style: &'static str| {
        view! {
            <div style=format!("display:flex;align-items:center;gap:0.5rem;padding:0.4rem 0.6rem;margin:0.1rem 0;border:1px solid var(--theme-surface-border);border-radius:var(--radius-sm);cursor:grab;background:var(--theme-page-bg);font-size:0.78rem;user-select:none;touch-action:none;{}", extra_style)>
                <span style="opacity:0.6;">{icon}</span>
                <span>{label}</span>
            </div>
        }
    };

    view! {
        <div style="width:220px;flex-shrink:0;border-right:1px solid var(--theme-surface-border);display:flex;flex-direction:column;background:var(--theme-surface-bg);" data-builder-panel="sidebar">

            // Tabs header
            <div style="border-bottom:1px solid var(--theme-surface-border);">
                <div style="display:flex;">
                    <button
                        style=move || tab_style(0, true)
                        on:click=move |_| step.set(0)
                    >"📐 Layout"</button>
                    <button
                        style=move || tab_style(1, has_layout.get())
                        on:click=move |_| { if has_layout.get() { step.set(1); } }
                    >"🧱 Blocks"</button>
                    <button
                        style=move || tab_style(2, has_blocks.get())
                        on:click=move |_| { if has_blocks.get() { step.set(2); } }
                    >"🎨 UI"</button>
                </div>
                // Hint
                <div style="padding:0.3rem 0.75rem;font-size:0.6rem;color:var(--theme-surface-fg-muted);border-top:1px solid var(--theme-surface-border);">
                    {move || match step.get() {
                        0 => "Drag a layout to the canvas",
                        1 => "Drag blocks into layout regions",
                        _ => "Drag UI components into blocks",
                    }}
                </div>
            </div>

            // Conteúdo
            <div style="flex:1;overflow-y:auto;padding:0.4rem;">

                // Step 0 — Layouts (draggáveis)
                {move || (step.get() == 0).then(|| view! {
                    <For
                        each=move || ActiveLayout::all()
                        key=|l| format!("{:?}", l)
                        children=move |layout| {
                            let label = layout.label();
                            let icon = layout.icon();
                            let desc = layout.description();
                            view! {
                                <div
                                    on:pointerdown=move |ev| {
                                        ev.prevent_default();
                                        ev.stop_propagation();
                                        drag_ctx.set(DragContext { node_id: None, block_def: None, component_def: None, layout_def: Some(layout) });
                                    }
                                    title=desc
                                    style="padding:0.6rem 0.75rem;margin:0.1rem 0;border:1px solid var(--theme-surface-border);border-radius:var(--radius-sm);cursor:pointer;background:var(--theme-page-bg);user-select:none;"
                                >
                                    <div style="font-size:1rem;">{icon}</div>
                                    <div style="font-size:0.78rem;font-weight:600;margin-top:0.15rem;">{label}</div>
                                    <div style="font-size:0.65rem;color:var(--theme-surface-fg-muted);margin-top:0.1rem;">{desc}</div>
                                </div>
                            }
                        }
                    />
                })}

                // Step 1 — Blocks
                {move || (step.get() == 1).then(|| view! {
                    <For
                        each=move || blocks_list.get()
                        key=|b| b.id
                        children=move |block| {
                            let id = block.id;
                            let label = block.label;
                            let icon = block.icon;
                            view! {
                                <div
                                    on:pointerdown=move |ev| {
                                        ev.prevent_default();
                                        leptos::logging::log!("[block pointerdown] id={:?}", id);
                                        if let Some(b) = super::domain::blocks::get_block(id).cloned() {
                                            drag_ctx.set(DragContext { node_id: None, block_def: Some(b), component_def: None, layout_def: None });
                                        }
                                    }
                                    style="display:flex;align-items:center;gap:0.5rem;padding:0.4rem 0.6rem;margin:0.1rem 0;border:1px solid var(--theme-surface-border);border-radius:var(--radius-sm);cursor:grab;background:var(--theme-page-bg);font-size:0.78rem;user-select:none;touch-action:none;"
                                >
                                    <span style="opacity:0.6;">{icon}</span>
                                    <span>{label}</span>
                                </div>
                            }
                        }
                    />
                })}

                // Step 2 — UI Components
                {move || (step.get() == 2).then(|| view! {
                    <For
                        each=move || components_list.get()
                        key=|c| c.id
                        children=move |comp| {
                            let id = comp.id;
                            let label = comp.label;
                            let icon = comp.icon;
                            let desc = comp.description;
                            view! {
                                <div
                                    on:pointerdown=move |ev| {
                                        ev.prevent_default();
                                        if let Some(c) = super::domain::blocks::get_component(id).cloned() {
                                            drag_ctx.set(DragContext { node_id: None, block_def: None, component_def: Some(c), layout_def: None });
                                        }
                                    }
                                    title=desc
                                    style="display:flex;align-items:center;gap:0.5rem;padding:0.4rem 0.6rem;margin:0.1rem 0;border:1px solid var(--theme-surface-border);border-radius:var(--radius-sm);cursor:grab;background:var(--theme-page-bg);font-size:0.78rem;user-select:none;touch-action:none;"
                                >
                                    <span style="opacity:0.6;">{icon}</span>
                                    <span>{label}</span>
                                </div>
                            }
                        }
                    />
                })}
            </div>
        </div>
    }
}
