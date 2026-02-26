use leptos::prelude::*;
use super::types::{Node, NodeKind};
use super::state::builder_engine::BuilderEngine;
use rs_canonrs::application::Command;
use canonrs_ui::ui::sheet::{Sheet, SheetContent, SheetOverlay, SheetSide};

#[component]
pub fn Inspector(
    engine: RwSignal<BuilderEngine>,
    tree: RwSignal<Vec<Node>>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
) -> impl IntoView {

    Effect::new(move |_| {
        let open = selected_id.get().is_some();
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(el) = doc.get_element_by_id("inspector-sheet") {
                        let state = if open { "open" } else { "closed" };
                        let _ = el.set_attribute("data-state", state);
                    }
                }
            }
        }
    });

    view! {
        <Sheet id="inspector-sheet" side=SheetSide::Right>
            <SheetOverlay />
            <SheetContent>
                {move || {
                    let sel = selected_id.get();
                    let node = sel.and_then(|id| tree.get().into_iter().find(|n| n.id == id));
                    match node {
                        None => view! { <div style="padding:1rem;font-size:0.75rem;opacity:0.5;">"No block selected"</div> }.into_any(),
                        Some(n) => {
                            let (label, category, is_container) = match &n.kind {
                                NodeKind::Block { def } => (def.label, format!("{:?}", def.category), def.is_container),
                                NodeKind::Slot { name } => (name.as_str(), "Slot".to_string(), true),
                                NodeKind::Region { label, .. } => (label.as_str(), "Region".to_string(), true),
                                NodeKind::Component { def } => (def.label, def.description.to_string(), false),
                                NodeKind::Text { variant, .. } => (variant.label(), variant.label().to_string(), false),
                        NodeKind::Layout { label, .. } => (label.as_str(), "Layout".to_string(), true),
                            };
                            let node_id = n.id;
                            let parent_id = n.parent_id;
                            view! {
                                <div style="display:flex;flex-direction:column;gap:0.6rem;padding:1rem;">
                                    <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:0.25rem;">
                                        <span style="font-weight:700;font-size:0.7rem;text-transform:uppercase;letter-spacing:0.08em;opacity:0.6;">"Inspector"</span>
                                        <button
                                            on:click=move |_| selected_id.set(None)
                                            style="background:none;border:none;cursor:pointer;font-size:1.2rem;line-height:1;opacity:0.5;"
                                        >"×"</button>
                                    </div>

                                    <div data-inspector-field="">
                                        <div style="font-size:0.7rem;opacity:0.5;margin-bottom:0.1rem;">"Block"</div>
                                        <div style="font-weight:600;font-size:0.85rem;">{label}</div>
                                    </div>

                                    <div data-inspector-field="">
                                        <div style="font-size:0.7rem;opacity:0.5;margin-bottom:0.1rem;">"Category"</div>
                                        <div style="font-family:monospace;font-size:0.8rem;">{category}</div>
                                    </div>

                                    <div data-inspector-field="">
                                        <div style="font-size:0.7rem;opacity:0.5;margin-bottom:0.1rem;">"Container"</div>
                                        <div style="font-size:0.8rem;">{if is_container { "Yes" } else { "No" }}</div>
                                    </div>

                                    <div data-inspector-field="">
                                        <div style="font-size:0.7rem;opacity:0.5;margin-bottom:0.1rem;">"Node ID"</div>
                                        <div style="font-family:monospace;font-size:0.6rem;word-break:break-all;opacity:0.6;">{node_id.to_string()}</div>
                                    </div>

                                    {parent_id.map(|pid| view! {
                                        <div data-inspector-field="">
                                            <div style="font-size:0.7rem;opacity:0.5;margin-bottom:0.1rem;">"Parent ID"</div>
                                            <div style="font-family:monospace;font-size:0.6rem;word-break:break-all;opacity:0.6;">{pid.to_string()}</div>
                                        </div>
                                    })}

                                    <div style="margin-top:0.5rem;padding-top:0.5rem;border-top:1px solid var(--theme-surface-border);">
                                        <button
                                            on:click=move |_| {
                                                engine.update(|e| { let _ = e.execute(Command::Remove { node_id }); });
                                                let flat = engine.get_untracked().sync_flat();
                                                tree.set(flat);
                                                selected_id.set(None);
                                            }
                                            style="width:100%;padding:0.4rem;font-size:0.75rem;border-radius:4px;border:1px solid #ef4444;background:transparent;color:#ef4444;cursor:pointer;"
                                        >"🗑 Remove block"</button>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                }}
            </SheetContent>
        </Sheet>
    }
}
