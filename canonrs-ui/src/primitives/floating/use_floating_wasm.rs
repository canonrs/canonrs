#[cfg(target_arch = "wasm32")]
use web_sys::{Element, HtmlElement, DomRect, window};
use wasm_bindgen::JsCast;
use leptos::prelude::*;
use super::types::*;

#[cfg(feature = "hydrate")]
pub fn use_floating_position(
    anchor_id: &str,
    floating_id: &str,
    config: FloatingConfig,
    active: ReadSignal<bool>,
) {
    use leptos::prelude::Effect;

    let anchor_id = anchor_id.to_string();
    let floating_id = floating_id.to_string();

    Effect::new(move |_| {
        if !active.get() {
            return;
        }

        let document = window()
            .expect("window not available")
            .document()
            .expect("document not available");

        let anchor: Element = match document.get_element_by_id(&anchor_id) {
            Some(el) => el,
            None => return,
        };

        let floating: Element = match document.get_element_by_id(&floating_id) {
            Some(el) => el,
            None => return,
        };

        let anchor_rect = anchor.get_bounding_client_rect();
        let floating_rect = floating.get_bounding_client_rect();

        let viewport_width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let viewport_height = window().unwrap().inner_height().unwrap().as_f64().unwrap();

        let position = calculate_position(
            &anchor_rect,
            &floating_rect,
            viewport_width,
            viewport_height,
            config,
        );

        if let Some(html_el) = floating.dyn_ref::<HtmlElement>() {
            let style = html_el.style();
            let _ = style.set_property("--floating-x", &format!("{}px", position.x));
            let _ = style.set_property("--floating-y", &format!("{}px", position.y));
            let _ = style.set_property("--floating-placement", position.placement.as_str());
        }
    });
}

fn calculate_position(
    anchor_rect: &DomRect,
    floating_rect: &DomRect,
    viewport_width: f64,
    viewport_height: f64,
    config: FloatingConfig,
) -> FloatingPosition {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut placement = config.placement;

    match config.placement {
        Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
            x = anchor_rect.left() + (anchor_rect.width() / 2.0) - (floating_rect.width() / 2.0);
            y = anchor_rect.bottom() + config.offset;

            if config.flip && y + floating_rect.height() > viewport_height {
                y = anchor_rect.top() - floating_rect.height() - config.offset;
                placement = Placement::Top;
            }
        }
        Placement::Top | Placement::TopStart | Placement::TopEnd => {
            x = anchor_rect.left() + (anchor_rect.width() / 2.0) - (floating_rect.width() / 2.0);
            y = anchor_rect.top() - floating_rect.height() - config.offset;

            if config.flip && y < 0.0 {
                y = anchor_rect.bottom() + config.offset;
                placement = Placement::Bottom;
            }
        }
        Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
            x = anchor_rect.left() - floating_rect.width() - config.offset;
            y = anchor_rect.top() + (anchor_rect.height() / 2.0) - (floating_rect.height() / 2.0);

            if config.flip && x < 0.0 {
                x = anchor_rect.right() + config.offset;
                placement = Placement::Right;
            }
        }
        Placement::Right | Placement::RightStart | Placement::RightEnd => {
            x = anchor_rect.right() + config.offset;
            y = anchor_rect.top() + (anchor_rect.height() / 2.0) - (floating_rect.height() / 2.0);

            if config.flip && x + floating_rect.width() > viewport_width {
                x = anchor_rect.left() - floating_rect.width() - config.offset;
                placement = Placement::Left;
            }
        }
    }

    x = x.max(0.0).min(viewport_width - floating_rect.width());
    y = y.max(0.0).min(viewport_height - floating_rect.height());

    FloatingPosition { x, y, placement }
}
