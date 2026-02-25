use leptos::prelude::*;
use crate::application::builder_controller::BuilderController;
use leptos::prelude::Effect;
use super::types::{ActiveLayout, Node, NodeKind, DragContext, CanvasMode, all_blocks, init_slots};
use super::state::builder_engine::BuilderEngine;
use rs_canonrs::domain::CanonDocument;
use rs_canonrs::application::Command;
use super::blocks_panel::BlocksPanel;
use super::layout_canvas::LayoutCanvas;
use super::inspector::Inspector;
use super::canvas_toolbar::CanvasToolbar;
use super::preview_protocol::{init_preview_listener, send_preview, PreviewState, retry_if_needed};
use super::ui::drop_zone::DragVisualState;

#[component]
pub fn BuilderWorkspace(
    controller: crate::application::builder_controller::BuilderController,
    active_layout: RwSignal<ActiveLayout>,
    slots: RwSignal<Vec<Node>>,
    tree: RwSignal<Vec<Node>>,
    engine: RwSignal<BuilderEngine>,
    drag_ctx: RwSignal<DragContext>,
    canvas_mode: RwSignal<CanvasMode>,
    drag_visual: RwSignal<DragVisualState>,
    viewport: RwSignal<crate::ui::theme_workspace::viewport::Viewport>,
) -> impl IntoView {
    use crate::ui::layout_builder::types::ActiveLayout;
    let selected_id: RwSignal<Option<uuid::Uuid>> = RwSignal::new(None);
    let preview_state = RwSignal::new(PreviewState::Idle);
    let last_doc_json: RwSignal<Option<String>> = RwSignal::new(None);
    let is_dirty: RwSignal<bool> = RwSignal::new(false);
    let last_saved_hash: RwSignal<Option<String>> = RwSignal::new(None);
    let controller = BuilderController::new(engine, tree, selected_id, is_dirty);

    // Inicializa protocolo READY/DOC/ACK — listener permanente
    #[cfg(target_arch = "wasm32")]
    init_preview_listener(slots, tree, active_layout, preview_state, last_doc_json);

    // Trigger de envio inicial — quando entra em Preview pela primeira vez
    // Update em tempo real (split view) será implementado na Fase 6

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

    let canvas_ref: NodeRef<leptos::html::Div> = NodeRef::new();



    // ── Keyboard Shortcuts ───────────────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    crate::infra::web_keyboard::register_keyboard_shortcuts(
        move || { controller.undo(); },
        move || { controller.redo(); },
        move || { controller.delete_selected(); },
    );

    // ── Pointer Listener ─────────────────────────────────────────────────────
    #[cfg(target_arch = "wasm32")]
    crate::infra::web_pointer::register_pointerup_listener(move |ev| {
        use leptos::wasm_bindgen::JsCast;
        leptos::logging::log!("[pointerup] fired, dragging={}", drag_ctx.get_untracked().is_dragging());
        if !drag_ctx.get_untracked().is_dragging() { return; }
        // Encontra drop zone sob o pointer
        let mut target_zone: Option<(uuid::Uuid, usize)> = None;
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                let el_at_point = doc.element_from_point(ev.client_x() as f32, ev.client_y() as f32);
                        leptos::logging::log!("[pointerup] element_from_point at ({},{}) = {:?}", ev.client_x(), ev.client_y(), el_at_point.as_ref().map(|e| e.tag_name()));
                        if let Some(el) = el_at_point {
                    let mut cur: web_sys::Element = el;
                        leptos::logging::log!("[pointerup] starting walk from tag={} has-drop-zone={}", cur.tag_name(), cur.has_attribute("data-drop-zone"));
                    loop {
                        if cur.has_attribute("data-drop-zone") {
                            if let Some(id_str) = cur.get_attribute("data-zone-id") {
                                if let Ok(uuid) = uuid::Uuid::parse_str(&id_str) {
                                    let idx = drag_visual.get_untracked().insert_index;
                                    target_zone = Some((uuid, idx));
                                }
                            }
                            break;
                        }
                        match cur.parent_element() { Some(p) => cur = p, None => break }
                    }
                }
            }
        }
        if let Some((parent_id, _)) = target_zone {
            use crate::ui::layout_builder::ui::drop_handler::handle_drop;
            handle_drop(ev, parent_id, engine, tree, drag_ctx, drag_visual);
        } else {
            use crate::ui::layout_builder::state::drop_zone_types::DragVisualState;
            use crate::ui::layout_builder::types::DragContext;
            batch(move || {
                drag_visual.set(DragVisualState::empty());
                drag_ctx.set(DragContext::empty());
            });
        }
    });

    crate::infra::web_pointer::register_pointer_listener(move |ev| {
        use leptos::wasm_bindgen::JsCast;
        if !drag_ctx.get_untracked().is_dragging() { return; }
        let mut new_active: Option<web_sys::Element> = None;
        if let Some(target) = ev.target() {
            let mut el: web_sys::Element = target.unchecked_into();
            loop {
                if el.has_attribute("data-drop-zone") { new_active = Some(el); break; }
                match el.parent_element() { Some(p) => el = p, None => break }
            }
        }
        if let Some(el) = new_active {
            let _ = el.set_attribute("data-drop-zone-active", "true");
            let client_y = ev.client_y() as f64;
            let mut idx = 0usize;
            if let Ok(blocks) = el.query_selector_all(":scope > [data-block-preview]") {
                let len = blocks.length() as usize; idx = len;
                let mut vi = 0usize;
                for i in 0..len {
                    if let Some(b) = blocks.item(i as u32) {
                        let be: web_sys::Element = b.unchecked_into();
                        if be.get_attribute("data-dragging").as_deref() == Some("true") { continue; }
                        let rect = be.get_bounding_client_rect();
                        if client_y < rect.top() + rect.height() / 2.0 { idx = vi; break; }
                        vi += 1; idx = vi;
                    }
                }
            }
            if let Some(zone_id_str) = el.get_attribute("data-zone-id") {
                if let Ok(zone_uuid) = uuid::Uuid::parse_str(&zone_id_str) {
                    drag_visual.set(DragVisualState { active_zone_id: Some(zone_uuid), insert_index: idx });
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
                                crate::infra::app_state::bootstrap_engine_with_layout(&layout);
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
                on:click=move |ev| {
                    use leptos::wasm_bindgen::JsCast;
                    if let Some(t) = ev.target() {
                        if t.unchecked_ref::<web_sys::Element>().closest("[data-block-preview]").ok().flatten().is_none() {
                            selected_id.set(None);
                        }
                    }
                }
            >
                <CanvasToolbar canvas_mode=canvas_mode slots=slots tree=tree active_layout=active_layout engine=engine is_dirty=is_dirty />
                {move || -> leptos::prelude::AnyView {
                    if canvas_mode.get() == CanvasMode::Preview {
                        view! {
                            <iframe id="canon-preview-frame" src="/preview"
                                style="width:100%;height:100%;border:none;min-height:600px;" />
                        }.into_any()
                    } else {
                        view! {
                            <div style=move || format!("margin:0 auto;width:{}px;min-height:{}px;transition:width 0.3s;overflow:hidden;", viewport.get().width, viewport.get().height)>
                                <LayoutCanvas
                                    layout=active_layout.get()
                                    engine=engine tree=tree drag_ctx=drag_ctx
                                    slots=slots selected_id=selected_id
                                    canvas_mode=canvas_mode drag_visual=drag_visual
                                />
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            <Inspector engine=engine tree=tree selected_id=selected_id />

            // ASIDE — Blocks
            <BlocksPanel tree=tree selected_id=selected_id drag_ctx=drag_ctx />
        </div>
    }
}
