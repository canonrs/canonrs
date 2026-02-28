use std::sync::OnceLock;

static POINTERMOVE_REGISTERED: OnceLock<()> = OnceLock::new();
static POINTERUP_REGISTERED: OnceLock<()> = OnceLock::new();

#[cfg(target_arch = "wasm32")]
pub fn register_pointer_listener<F>(handler: F) where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    if POINTERMOVE_REGISTERED.set(()).is_err() { return; }
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;
    leptos::task::spawn_local(async move {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                let closure = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
                    move |ev: web_sys::PointerEvent| { handler(ev); },
                );
                let _ = doc.add_event_listener_with_callback_and_bool(
                    "pointermove", closure.as_ref().unchecked_ref(), true,
                );
                closure.forget();
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
pub fn register_pointerup_listener<F>(handler: F) where
    F: Fn(web_sys::PointerEvent) + 'static,
{
    if POINTERUP_REGISTERED.set(()).is_err() { return; }
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;
    leptos::task::spawn_local(async move {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                let closure = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
                    move |ev: web_sys::PointerEvent| { handler(ev); },
                );
                let _ = doc.add_event_listener_with_callback_and_bool(
                    "pointerup", closure.as_ref().unchecked_ref(), true,
                );
                closure.forget();
            }
        }
    });
}

use leptos::prelude::*;
use crate::ui::layout_builder::state::drop_zone_types::{DragVisualState, RegionOrientation};
use crate::ui::layout_builder::types::{Node, CanvasMode};

const MIN_REGION_HEIGHT: f64 = 32.0;

#[derive(Clone, Debug, PartialEq)]
pub struct RegionBox {
    pub top: f64,
    pub left: f64,
    pub width: f64,
    pub height: f64,
    pub zone_id: String,
}

pub fn get_region_boxes(canvas_mode: CanvasMode) -> Vec<RegionBox> {
    if canvas_mode != CanvasMode::Builder && canvas_mode != CanvasMode::Wireframe {
        return vec![];
    }
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                let selector = match canvas_mode {
                    CanvasMode::Wireframe => "[data-canvas-mode='wireframe']",
                    _ => "[data-canvas-mode='builder']",
                };
                if let Ok(Some(canvas)) = doc.query_selector(selector) {
                    let canvas_rect = canvas.get_bounding_client_rect();
                    if let Ok(zones) = canvas.query_selector_all("[data-drop-zone][data-zone-id]") {
                        let mut boxes = vec![];
                        for i in 0..zones.length() {
                            if let Some(el) = zones.item(i) {
                                let zone_el: web_sys::Element = el.unchecked_into();
                                let zone_id = zone_el.get_attribute("data-zone-id").unwrap_or_default();

                                // Usa região pai para dimensão visual real
                                let measure_el = zone_el.parent_element()
                                    .unwrap_or_else(|| zone_el.clone());
                                let r = measure_el.get_bounding_client_rect();

                                let top = r.top() - canvas_rect.top();
                                let left = r.left() - canvas_rect.left();
                                let raw_width = r.right() - r.left();
                                let raw_height = r.bottom() - r.top();

                                // Garante hit area mínima mesmo com altura 0
                                let height = if raw_height < MIN_REGION_HEIGHT { MIN_REGION_HEIGHT } else { raw_height };
                                let width = if raw_width < 1.0 { 60.0 } else { raw_width };

                                boxes.push(RegionBox { top, left, width, height, zone_id });
                            }
                        }
                        return boxes;
                    }
                }
            }
        }
    }
    vec![]
}

#[component]
pub fn BuilderOverlay(
    drag_visual: RwSignal<DragVisualState>,
    canvas_mode: RwSignal<CanvasMode>,
    slots: RwSignal<Vec<Node>>,
    drag_ctx: RwSignal<crate::ui::layout_builder::types::DragContext>,
    tree: RwSignal<Vec<Node>>,
    engine: RwSignal<crate::ui::layout_builder::state::builder_engine::BuilderEngine>,
    selected_id: RwSignal<Option<uuid::Uuid>>,
) -> impl IntoView {
    let tick = RwSignal::new(0u32);

    Effect::new(move |_| {
        let _ = slots.get();
        let _ = tree.get();
        let _ = drag_ctx.get();
        tick.update(|t| *t += 1);
    });

    let region_boxes = Memo::new(move |_| {
        let _ = tick.get();
        let _ = drag_visual.get();
        get_region_boxes(canvas_mode.get())
    });

    let is_wireframe = move || canvas_mode.get() == CanvasMode::Wireframe;
    let is_dragging  = move || drag_ctx.get().is_dragging();

    let marker_style = move || {
        let vs = drag_visual.get();
        if let Some(rect) = vs.region_rect {
            match vs.orientation {
                RegionOrientation::Column => format!(
                    "position:absolute;top:{top}px;left:{left}px;width:{width}px;height:4px;\
                     background:var(--builder-insert-line-color);border-radius:2px;\
                     pointer-events:none;z-index:52;transition:top 80ms ease;",
                    top = rect.top + vs.insert_pos, left = rect.left, width = rect.width,
                ),
                RegionOrientation::Row => format!(
                    "position:absolute;top:{top}px;left:{left}px;width:4px;height:{height}px;\
                     background:var(--builder-insert-line-color);border-radius:2px;\
                     pointer-events:none;z-index:52;transition:left 80ms ease;",
                    top = rect.top, left = rect.left + vs.insert_pos, height = rect.height,
                ),
            }
        } else { "display:none;".to_string() }
    };

    view! {
        <div style="position:absolute;inset:0;pointer-events:none;z-index:9999;overflow:visible;">

            {move || {
                let boxes = region_boxes.get();
                let active = drag_visual.get().active_zone_id.map(|u| u.to_string());
                let wireframe = is_wireframe();
                let dragging = is_dragging();

                boxes.into_iter().map(move |r| {
                        let zone_id_str = r.zone_id.clone();
                    let zone_id_str2 = r.zone_id.clone();

                    let base_style = format!(
                        "position:absolute;top:{top}px;left:{left}px;width:{width}px;height:{height}px;\
                         border-radius:2px;z-index:50;",
                        top = r.top, left = r.left, width = r.width, height = r.height,
                    );

                    let zone_id_for_style = r.zone_id.clone();
                    let visual_style = move || {
                        let is_drag = drag_ctx.get().is_dragging();
                        let act_id = drag_visual.get().active_zone_id.map(|u| u.to_string());
                        let is_act = act_id.as_deref() == Some(zone_id_for_style.as_str());
                        if wireframe {
                            format!("{base_style}border:1px solid var(--builder-region-border);background:transparent;pointer-events:none;")
                        } else if is_drag {
                            format!(
                                "{base_style}border:2px dashed {border};background:{bg};pointer-events:auto;cursor:copy;                                 transition:border-color 150ms ease,background 150ms ease;{pulse}",
                                border = if is_act { "var(--builder-insert-line-color)" } else { "var(--builder-region-border)" },
                                bg = if is_act { "var(--builder-region-bg-active)" } else { "rgba(0,0,0,0.01)" },
                                pulse = if is_act { "animation:region-pulse var(--builder-pulse-duration) ease infinite;" } else { "" },
                            )
                        } else {
                            format!("{base_style}border:2px dashed var(--builder-region-border);background:transparent;pointer-events:none;")
                        }
                    };

                    // Overlay intercepta pointermove/pointerup quando arrastando
                    view! {
                        <div
                            style=visual_style
                            attr:data-overlay-zone=zone_id_str.clone()
                            on:pointermove=move |ev: leptos::ev::PointerEvent| {
                                if !drag_ctx.get_untracked().is_dragging() { return; }
                                ev.stop_propagation();
                                let client_y = ev.client_y() as f64;
                                let mut idx = 0usize;

                                // Busca blocos dentro do drop-zone real
                                #[cfg(target_arch = "wasm32")] {
                                    use leptos::wasm_bindgen::JsCast;
                                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                                        let sel = format!("[data-drop-zone][data-zone-id='{}']", zone_id_str);
                                        if let Ok(Some(zone_el)) = doc.query_selector(&sel) {
                                            if let Ok(blocks) = zone_el.query_selector_all(":scope > [data-block-preview]") {
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
                                        }
                                    }
                                }

                                if let Ok(zone_uuid) = uuid::Uuid::parse_str(&zone_id_str) {
                                    drag_visual.set(DragVisualState {
                                        active_zone_id: Some(zone_uuid),
                                        insert_index: idx,
                                        region_rect: None,
                                        orientation: RegionOrientation::Column,
                                        insert_pos: 0.0,
                                    });
                                }
                            }
                            on:pointerup=move |ev: leptos::ev::PointerEvent| {
                                if !drag_ctx.get_untracked().is_dragging() { return; }
                                ev.stop_propagation();
                                if let Ok(zone_uuid) = uuid::Uuid::parse_str(&zone_id_str2) {
                                    let vs = drag_visual.get_untracked();
                                    let idx = vs.insert_index;
                                    let idx = drag_visual.get_untracked().insert_index;
                                    super::ui::drop_handler::execute_drop(
                                        zone_uuid, idx, engine, tree, drag_ctx, drag_visual,
                                    );
                                }
                                drag_visual.set(DragVisualState::empty());
                            }
                        />
                    }
                }).collect::<Vec<_>>()
            }}

            {move || drag_visual.get().region_rect.is_some().then(|| view! {
                <div style=marker_style />
            })}
        </div>
    }
}
