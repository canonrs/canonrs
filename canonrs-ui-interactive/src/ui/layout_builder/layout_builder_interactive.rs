use leptos::prelude::*;
use leptos::prelude::Effect;
use super::types::{ActiveLayout, Node, DragContext, CanvasMode, all_blocks, init_slots};
use super::layout_canvas::LayoutCanvas;
use super::inspector::Inspector;
use super::canvas_toolbar::CanvasToolbar;

#[component]
pub fn LayoutBuilderInteractive() -> impl IntoView {
    let active_layout = RwSignal::new(ActiveLayout::Dashboard);
    let slots: RwSignal<Vec<Node>> = RwSignal::new(init_slots(&ActiveLayout::Dashboard));
    let tree: RwSignal<Vec<Node>> = RwSignal::new(vec![]);
    let drag_ctx = RwSignal::new(DragContext::empty());
    let selected_id: RwSignal<Option<uuid::Uuid>> = RwSignal::new(None);
    let canvas_mode = RwSignal::new(CanvasMode::Builder);

    // Cursor global durante drag
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;
            if let Some(doc) = leptos::leptos_dom::helpers::document().body() {
                if drag_ctx.get().is_dragging() {
                    let _ = doc.style().set_property("cursor", "grabbing");
                    let _ = doc.style().set_property("user-select", "none");
                } else {
                    let _ = doc.style().remove_property("cursor");
                    let _ = doc.style().remove_property("user-select");
                }
            }
        }
    });

    view! {
        <div
            data-interactive="layout-builder"
            style="display: flex; height: 100%; min-height: 600px; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;"
        >
            // SIDEBAR — Layout thumbnails
            <div style="width: 220px; flex-shrink: 0; border-right: 1px solid var(--theme-surface-border); overflow-y: auto; background: var(--theme-surface-bg);" data-builder-panel="layouts">
                <div style="padding: 0.75rem 1rem; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--theme-surface-fg-muted); border-bottom: 1px solid var(--theme-surface-border);">
                    "Layouts"
                </div>
                {ActiveLayout::all().into_iter().map(|layout| {
                    let is_active = move || active_layout.get() == layout;
                    view! {
                        <button
                            on:click=move |_| {
                                active_layout.set(layout);
                                slots.set(init_slots(&layout));
                                tree.set(vec![]);
                            }
                            style=move || format!(
                                "width: 100%; text-align: left; padding: 0.75rem 1rem; border: none; cursor: pointer; border-bottom: 1px solid var(--theme-surface-border); background: {}; transition: background 0.15s;",
                                if is_active() { "var(--theme-primary-bg)" } else { "transparent" }
                            )
                        >
                            <div style="font-size: 1.25rem; margin-bottom: 0.25rem;">{layout.icon()}</div>
                            <div style=move || format!(
                                "font-size: 0.8rem; font-weight: {}; color: {};",
                                if is_active() { "600" } else { "400" },
                                if is_active() { "var(--theme-primary-fg)" } else { "var(--theme-surface-fg)" }
                            )>{layout.label()}</div>
                            <div style="font-size: 0.7rem; color: var(--theme-surface-fg-muted); margin-top: 0.1rem;">{layout.description()}</div>
                        </button>
                    }
                }).collect_view()}
            </div>

            // CANVAS
            <div
                style="flex: 1; overflow: auto; padding: 1rem; background: var(--theme-page-bg);"
                data-builder-panel="canvas"
                on:click=move |ev| { use leptos::wasm_bindgen::JsCast; if let Some(t) = ev.target() { if t.unchecked_ref::<web_sys::Element>().closest("[data-block-preview]").ok().flatten().is_none() { selected_id.set(None); } } }
            >
                <CanvasToolbar canvas_mode=canvas_mode />
                {move || view! {
                    <LayoutCanvas
                        layout=active_layout.get()
                        tree=tree
                        drag_ctx=drag_ctx
                        slots=slots
                        selected_id=selected_id
                        canvas_mode=canvas_mode
                    />
                }}
            </div>

            <Inspector tree=tree selected_id=selected_id />

            // ASIDE — Blocks
            <div style="width: 200px; flex-shrink: 0; border-left: 1px solid var(--theme-surface-border); overflow-y: auto; background: var(--theme-surface-bg);" data-builder-panel="blocks">
                <div style="padding: 0.75rem 1rem; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--theme-surface-fg-muted); border-bottom: 1px solid var(--theme-surface-border);">
                    "Blocks"
                </div>
                <div style="padding: 0.5rem;">
                    {all_blocks().into_iter().map(|block| {
                        let id = block.id;
                        let label = block.label;
                        let icon = block.icon;
                        view! {
                            <div
                                on:pointerdown=move |ev| {
                                    ev.prevent_default();
                                    if let Some(block) = all_blocks().into_iter().find(|b| b.id == id) {
                                        drag_ctx.set(DragContext {
                                            node_id: None,
                                            block_def: Some(block),
                                        });
                                    }
                                }
                                style="display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 0.75rem; margin-bottom: 0.25rem; border: 1px solid var(--theme-surface-border); border-radius: var(--radius-sm); cursor: grab; background: var(--theme-page-bg); font-size: 0.8rem; user-select: none; touch-action: none;"
                                data-builder-block=id
                            >
                                <span>{icon}</span><span>{label}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}
