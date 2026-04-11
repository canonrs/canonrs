//! Positioning — respeita data-rs-side do SSR, faz flip apenas se nao ha espaco

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

pub fn auto_side(root: &Element, content_selector: &str) {
    let Some(win) = web_sys::window() else { return };
    let Ok(Some(content)) = root.query_selector(content_selector) else { return };

    if let Ok(root_html) = root.clone().dyn_into::<HtmlElement>() {
        let _ = root_html.style().set_property("position", "relative");
    }

    let preferred = content.get_attribute("data-rs-side").unwrap_or_else(|| "bottom".to_string());

    let content_rect = content.get_bounding_client_rect();
    let root_rect = root.get_bounding_client_rect();
    let vh = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(768.0);
    let vw = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1024.0);
    let ch = content_rect.height();
    let cw = content_rect.width();

    if ch == 0.0 && cw == 0.0 { return; }

    let has_space_bottom = vh - root_rect.bottom() >= ch + 8.0;
    let has_space_top    = root_rect.top()          >= ch + 8.0;
    let has_space_right  = vw - root_rect.right()   >= cw + 8.0;
    let has_space_left   = root_rect.left()          >= cw + 8.0;

    let preferred_has_space = match preferred.as_str() {
        "bottom" => has_space_bottom,
        "top"    => has_space_top,
        "right"  => has_space_right,
        "left"   => has_space_left,
        _        => has_space_bottom,
    };

    let side = if preferred_has_space {
        preferred
    } else if has_space_bottom { "bottom".to_string() }
    else if has_space_top      { "top".to_string() }
    else if has_space_right    { "right".to_string() }
    else                       { "left".to_string() };

    let _ = content.set_attribute("data-rs-side", &side);
}
