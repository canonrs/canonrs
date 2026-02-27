use leptos::prelude::*;
use crate::application::builder_controller::BuilderController;
use super::types::{ActiveLayout, Node, DragContext, CanvasMode};
use super::state::builder_engine::BuilderEngine;
use super::blocks_panel::BlocksPanel;
use super::layout_canvas::LayoutCanvas;
use super::builder_overlay::BuilderOverlay;
use super::inspector::Inspector;
use super::canvas_toolbar::CanvasToolbar;
use super::preview_protocol::{init_preview_listener, PreviewState};
use super::ui::drop_zone::DragVisualState;

#[component]
pub fn BuilderWorkspace(
    controller: crate::application::builder_controller::BuilderController,
    active_layout: RwSignal<Option<ActiveLayout>>,
    slots: RwSignal<Vec<Node>>,
    tree: RwSignal<Vec<Node>>,
    engine: RwSignal<BuilderEngine>,
    drag_ctx: RwSignal<DragContext>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<DragVisualState>,
    viewport: RwSignal<crate::ui::theme_workspace::viewport::Viewport>,
) -> impl IntoView {
    let selected_id: RwSignal<Option<uuid::Uuid>> = RwSignal::new(None);
    let preview_state = RwSignal::new(PreviewState::Idle);
    let last_doc_json: RwSignal<Option<String>> = RwSignal::new(None);
    let is_dirty: RwSignal<bool> = RwSignal::new(false);
    let controller = BuilderController::new(engine, tree, selected_id, is_dirty);

    #[cfg(target_arch = "wasm32")]
    init_preview_listener(slots, tree, active_layout, preview_state, last_doc_json);

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

    #[cfg(target_arch = "wasm32")]
    crate::infra::web_keyboard::register_keyboard_shortcuts(
        move || { controller.undo(); },
        move || { controller.redo(); },
        move || { controller.delete_selected(); },
    );

    view! {
        <div style="display:flex;height:100%;min-height:600px;overflow:hidden;">

            <BlocksPanel tree=tree selected_id=selected_id drag_ctx=drag_ctx active_layout=active_layout engine=engine />

            <div style="flex:1;display:flex;flex-direction:column;overflow:hidden;">
                <CanvasToolbar canvas_mode=canvas_mode slots=slots tree=tree active_layout=active_layout engine=engine is_dirty=is_dirty />
                <div
                    style="flex:1;overflow:auto;background:#e5e7eb;display:flex;justify-content:center;padding:2rem;"
                    on:click=move |ev| {
                        use leptos::wasm_bindgen::JsCast;
                        if let Some(t) = ev.target() {
                            let el = t.unchecked_ref::<web_sys::Element>();
                            if el.closest("[data-block-preview]").ok().flatten().is_none() {
                                // Seleciona nó Layout se existir
                                let layout_id = tree.get_untracked().iter()
                                    .find(|n| matches!(&n.kind, super::types::NodeKind::Layout { .. }))
                                    .map(|n| n.id);
                                selected_id.set(layout_id);
                            }
                        }
                    }
                >
                    {move || -> leptos::prelude::AnyView {
                        if canvas_mode.get() == CanvasMode::Preview {
                            view! {
                                <iframe id="canon-preview-frame" src="/preview"
                                    style="width:100%;height:100%;border:none;min-height:600px;" />
                            }.into_any()
                        } else if active_layout.get().is_none() {
                            view! {
                                <div style="display:flex;align-items:center;justify-content:center;height:60vh;color:#94a3b8;font-size:1rem;">
                                    "← Drop a layout to get started"
                                </div>
                            }.into_any()
                        } else {
                            let layout = active_layout.get().unwrap_or(crate::ui::layout_builder::domain::layout::ActiveLayout::Dashboard);
                            view! {
                                <div
                                    data-canvas-mode=move || match canvas_mode.get() { CanvasMode::Builder => "builder", CanvasMode::Preview => "preview", CanvasMode::Wireframe => "wireframe" }
                                    attr:data-layout-dragging=move || if drag_ctx.get().layout_def.is_some() { "true" } else { "false" }
                                    style=move || format!("position:relative;width:{}px;min-height:{}px;box-shadow:0 4px 24px rgba(0,0,0,0.12);border-radius:8px;overflow:visible;color-scheme:light;--theme-surface-bg:#ffffff;--theme-surface-fg:#0f172a;--theme-surface-border:#e2e8f0;--color-bg-surface:#ffffff;--color-bg-muted:#f8fafc;--color-border-default:#e2e8f0;{}", viewport.get().width, viewport.get().height, if drag_ctx.get().layout_def.is_some() { "opacity:0.15;pointer-events:none;" } else { "" })
                                >
                                        <BuilderOverlay drag_visual=drag_visual canvas_mode=canvas_mode slots=slots />
                                        <button
                                            on:click=move |_| {
                                                active_layout.set(None);
                                                crate::infra::app_state::global_slots().set(vec![]);
                                                crate::infra::app_state::global_tree().set(vec![]);
                                            }
                                            style="position:absolute;top:8px;right:8px;z-index:200;width:24px;height:24px;border-radius:50%;border:2px solid white;cursor:pointer;background:#ef4444;color:white;font-size:0.75rem;font-weight:700;display:flex;align-items:center;justify-content:center;line-height:1;"
                                            title="Remove layout"
                                        >
                                            "✕"
                                        </button>
                
                                        {move || {
                                            let layout = active_layout.get().unwrap_or(crate::ui::layout_builder::domain::layout::ActiveLayout::Dashboard);
                                            view! {
                                                <LayoutCanvas
                                                    layout=layout
                                                    engine=engine tree=tree drag_ctx=drag_ctx
                                                    slots=slots selected_id=selected_id
                                                    canvas_mode=canvas_mode drag_visual=drag_visual
                                                />
                                            }
                                        }}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </div>

            <Inspector engine=engine tree=tree selected_id=selected_id />
        </div>
    }
}
