use leptos::prelude::*;
use super::types::{Node, NodeKind, DragContext, all_blocks, all_components};

#[component]
pub fn BlocksPanel(
    tree: RwSignal<Vec<Node>>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
    drag_ctx: RwSignal<DragContext>,
) -> impl IntoView {
    let sidebar_tab: RwSignal<u8> = RwSignal::new(0);

    // Contexto reativo — só muda quando selected_id muda
    let sidebar_context = Memo::new(move |_| {
        let sel_id = selected_id.get();
        sel_id.and_then(|id| tree.get_untracked().into_iter().find(|n| n.id == id))
            .and_then(|n| match &n.kind {
                NodeKind::Region { block_id, region_id, label } =>
                    Some((block_id.to_string(), region_id.to_string(), label.to_string())),
                _ => None,
            })
    });

    // Lista de blocks filtrada — Memo recalcula só quando contexto muda
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

    // Lista de components filtrada — Memo
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

    view! {
        <div
            style="width: 220px; flex-shrink: 0; border-left: 1px solid var(--theme-surface-border); overflow-y: auto; background: var(--theme-surface-bg);"
            data-builder-panel="blocks"
        >
            // Header contextual
            <div style="padding: 0.6rem 0.75rem; border-bottom: 1px solid var(--theme-surface-border);">
                <div style="font-size: 0.6rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 0.25rem;">
                    {move || match sidebar_context.get() {
                        Some((_, _, label)) => view! {
                            <span style="color: var(--theme-action-primary-bg);">
                                "Into: " {label}
                            </span>
                        }.into_any(),
                        None => view! {
                            <span style="color: var(--theme-surface-fg-muted);">
                                "All components"
                            </span>
                        }.into_any(),
                    }}
                </div>
                // Abas
                <div style="display: flex; gap: 0.25rem;">
                    <button
                        on:click=move |_| sidebar_tab.set(0)
                        style=move || format!(
                            "flex:1;padding:0.25rem;font-size:0.7rem;font-weight:600;border-radius:var(--radius-sm);border:1px solid {};background:{};color:{};cursor:pointer;",
                            if sidebar_tab.get()==0 {"var(--theme-action-primary-bg)"} else {"var(--theme-surface-border)"},
                            if sidebar_tab.get()==0 {"var(--theme-action-primary-bg)"} else {"transparent"},
                            if sidebar_tab.get()==0 {"white"} else {"var(--theme-surface-fg)"},
                        )
                    >"Blocks"</button>
                    <button
                        on:click=move |_| sidebar_tab.set(1)
                        style=move || format!(
                            "flex:1;padding:0.25rem;font-size:0.7rem;font-weight:600;border-radius:var(--radius-sm);border:1px solid {};background:{};color:{};cursor:pointer;",
                            if sidebar_tab.get()==1 {"var(--theme-action-primary-bg)"} else {"var(--theme-surface-border)"},
                            if sidebar_tab.get()==1 {"var(--theme-action-primary-bg)"} else {"transparent"},
                            if sidebar_tab.get()==1 {"white"} else {"var(--theme-surface-fg)"},
                        )
                    >"UI"</button>
                </div>
            </div>

            // Lista
            <div style="padding: 0.4rem;">
                <Show
                    when=move || sidebar_tab.get() == 0
                    fallback=move || view! {
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
                                                drag_ctx.set(DragContext { node_id: None, block_def: None, component_def: Some(c) });
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
                    }
                >
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
                                        if let Some(b) = super::domain::blocks::get_block(id).cloned() {
                                            drag_ctx.set(DragContext { node_id: None, block_def: Some(b), component_def: None });
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
                </Show>
            </div>
        </div>
    }
}
