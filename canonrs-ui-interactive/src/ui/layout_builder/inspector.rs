use leptos::prelude::*;
use super::types::{Node, NodeKind};
use super::state::builder_engine::BuilderEngine;
use rs_canonrs::application::Command;

#[component]
pub fn Inspector(
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
) -> impl IntoView {
    view! {
        <div
            style="width: 220px; flex-shrink: 0; border-left: 1px solid var(--theme-surface-border); overflow-y: auto; background: var(--theme-surface-bg);"
            data-builder-panel="inspector"
        >
            <div style="padding: 0.75rem 1rem; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--theme-surface-fg-muted); border-bottom: 1px solid var(--theme-surface-border);">
                "Inspector"
            </div>
            {move || {
                let sel = selected_id.get();
                let node = sel.and_then(|id| tree.get().into_iter().find(|n| n.id == id));
                match node {
                    None => view! {
                        <div style="padding: 1rem; font-size: 0.75rem; color: var(--theme-surface-fg-muted); text-align: center;">
                            "Select a block to inspect"
                        </div>
                    }.into_any(),
                    Some(n) => {
                        let (label, category, is_container) = match &n.kind {
                            NodeKind::Block { def } => (def.label, format!("{:?}", def.category), def.is_container),
                            NodeKind::Slot { name } => (name.as_str(), "Slot".to_string(), true),
                            NodeKind::Region { label, .. } => (label.as_str(), "Region".to_string(), true),
                            NodeKind::Component { def } => (def.label, def.description.to_string(), false),
                            NodeKind::Text { variant, content } => (variant.label(), variant.label().to_string(), false),
                        };
                        let node_id = n.id;
                        let parent_id = n.parent_id;
                        let index = 0usize; // index removido
                        view! {
                            <div style="padding: 0.75rem 1rem; display: flex; flex-direction: column; gap: 0.5rem;">

                                <div data-inspector-field="">
                                    <div data-inspector-label="">"Block"</div>
                                    <div data-inspector-value="" style="font-weight: 600;">{label}</div>
                                </div>

                                <div data-inspector-field="">
                                    <div data-inspector-label="">"Category"</div>
                                    <div data-inspector-value="" style="font-family: monospace;">{category}</div>
                                </div>

                                <div data-inspector-field="">
                                    <div data-inspector-label="">"Container"</div>
                                    <div data-inspector-value="">{if is_container { "Yes" } else { "No" }}</div>
                                </div>

                                <div data-inspector-field="">
                                    <div data-inspector-label="">"Index"</div>
                                    <div data-inspector-value="" style="font-family: monospace;">{index.to_string()}</div>
                                </div>

                                <div data-inspector-field="">
                                    <div data-inspector-label="">"Node ID"</div>
                                    <div data-inspector-value="" style="font-family: monospace; font-size: 0.6rem; word-break: break-all; color: var(--theme-surface-fg-muted);">{node_id.to_string()}</div>
                                </div>

                                {if parent_id.is_some() {
                                    Some(view! {
                                        <div data-inspector-field="">
                                            <div data-inspector-label="">"Parent ID"</div>
                                            <div data-inspector-value="" style="font-family: monospace; font-size: 0.6rem; word-break: break-all; color: var(--theme-surface-fg-muted);">{parent_id.unwrap().to_string()}</div>
                                        </div>
                                    })
                                } else { None }}

                                <div style="margin-top: 0.5rem; padding-top: 0.5rem; border-top: 1px solid var(--theme-surface-border);">
                                    <button
                                        on:click=move |_| {
                                            engine.update(|e| { let _ = e.execute(Command::Remove { node_id }); });
                                            let flat = engine.get_untracked().sync_flat();
                                            tree.set(flat);
                                            selected_id.set(None);
                                        }
                                        style="width: 100%; padding: 0.4rem; font-size: 0.75rem; border-radius: var(--radius-sm); border: 1px solid var(--theme-destructive-bg, #ef4444); background: transparent; color: var(--theme-destructive-bg, #ef4444); cursor: pointer;"
                                    >
                                        "Remove block"
                                    </button>
                                </div>

                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
