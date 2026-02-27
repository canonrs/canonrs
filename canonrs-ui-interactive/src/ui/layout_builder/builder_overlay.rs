use leptos::prelude::*;
use crate::ui::layout_builder::state::drop_zone_types::{DragVisualState, RegionOrientation};
use crate::ui::layout_builder::types::{Node, CanvasMode};

#[derive(Clone, Debug, PartialEq)]
struct RegionBox {
    top: f64,
    left: f64,
    width: f64,
    height: f64,
    zone_id: String,
}

fn get_region_boxes(canvas_mode: CanvasMode) -> Vec<RegionBox> {
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
                    if let Ok(regions) = canvas.query_selector_all("[data-drop-zone][data-zone-id]") {
                        let mut boxes = vec![];
                        for i in 0..regions.length() {
                            if let Some(el) = regions.item(i) {
                                let el: web_sys::Element = el.unchecked_into();
                                let r = el.get_bounding_client_rect();
                                let zone_id = el.get_attribute("data-zone-id").unwrap_or_default();
                                let top = r.top() - canvas_rect.top();
                                let left = r.left() - canvas_rect.left();
                                let bottom = r.bottom() - canvas_rect.top();
                                let right = r.right() - canvas_rect.left();
                                boxes.push(RegionBox {
                                    top,
                                    left,
                                    width: right - left,
                                    height: bottom - top,
                                    zone_id,
                                });
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
) -> impl IntoView {
    let tick = RwSignal::new(0u32);

    Effect::new(move |_| {
        let _ = slots.get();
        tick.update(|t| *t += 1);
    });

    let region_boxes = Memo::new(move |_| {
        let _ = tick.get();
        let _ = drag_visual.get();
        get_region_boxes(canvas_mode.get())
    });

    let is_wireframe = move || canvas_mode.get() == CanvasMode::Wireframe;

    let marker_style = move || {
        let vs = drag_visual.get();
        if let Some(rect) = vs.region_rect {
            match vs.orientation {
                RegionOrientation::Column => format!(
                    "position:absolute;top:{top}px;left:{left}px;width:{width}px;height:4px;\
                     background:var(--builder-insert-line-color);border-radius:2px;\
                     pointer-events:none;z-index:52;transition:top 80ms ease;",
                    top = rect.top + vs.insert_pos,
                    left = rect.left,
                    width = rect.width,
                ),
                RegionOrientation::Row => format!(
                    "position:absolute;top:{top}px;left:{left}px;width:4px;height:{height}px;\
                     background:var(--builder-insert-line-color);border-radius:2px;\
                     pointer-events:none;z-index:52;transition:left 80ms ease;",
                    top = rect.top,
                    left = rect.left + vs.insert_pos,
                    height = rect.height,
                ),
            }
        } else {
            "display:none;".to_string()
        }
    };

    view! {
        <div style="position:absolute;inset:0;pointer-events:none;z-index:9999;overflow:visible;">

            {move || {
                let boxes = region_boxes.get();
                let active = drag_visual.get().active_zone_id.map(|u| u.to_string());
                let wireframe = is_wireframe();
                boxes.into_iter().map(|r| {
                    let is_active = active.as_deref() == Some(r.zone_id.as_str());
                    let style = if wireframe {
                        format!(
                            "position:absolute;top:{top}px;left:{left}px;width:{width}px;height:{height}px;\
                             border:1px solid var(--builder-region-border);\
                             background:transparent;\
                             border-radius:2px;pointer-events:none;",
                            top = r.top, left = r.left, width = r.width, height = r.height,
                        )
                    } else {
                        format!(
                            "position:absolute;top:{top}px;left:{left}px;width:{width}px;height:{height}px;\
                             border:2px dashed {border};\
                             background:{bg};\
                             border-radius:2px;\
                             pointer-events:none;\
                             transition:border-color 150ms ease,background 150ms ease;\
                             {pulse}",
                            top = r.top,
                            left = r.left,
                            width = r.width,
                            height = r.height,
                            border = if is_active { "var(--builder-insert-line-color)" } else { "var(--builder-region-border)" },
                            bg = if is_active { "var(--builder-region-bg-active)" } else { "transparent" },
                            pulse = if is_active { "animation:region-pulse var(--builder-pulse-duration) ease infinite;" } else { "" },
                        )
                    };
                    view! { <div style=style /> }
                }).collect::<Vec<_>>()
            }}

            {move || drag_visual.get().region_rect.is_some().then(|| view! {
                <div style=marker_style />
            })}

        </div>
    }
}
