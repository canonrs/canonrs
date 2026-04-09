//! Positioning — auto-flip baseado em viewport
//! Flip so ocorre se o content tem dimensoes reais mensuráveis

use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn auto_side(root: &Element, content_selector: &str) {
    let Some(win) = web_sys::window() else { return };
    let Ok(Some(content)) = root.query_selector(content_selector) else { return };

    if let Ok(root_html) = root.clone().dyn_into::<web_sys::HtmlElement>() {
        let _ = root_html.style().set_property("position", "relative");
    }

    let content_rect = content.get_bounding_client_rect();

    // Sem dimensoes reais — content ainda invisible, nao tentar flip
    if content_rect.height() == 0.0 && content_rect.width() == 0.0 {
        let _ = content.set_attribute("data-rs-side", "bottom");
        return;
    }

    let root_rect = root.get_bounding_client_rect();
    let vh = win.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(768.0);
    let vw = win.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1024.0);
    let ch = content_rect.height();
    let cw = content_rect.width();

    let side = if vh - root_rect.bottom() >= ch + 8.0 { "bottom" }
               else if root_rect.top()    >= ch + 8.0 { "top" }
               else if vw - root_rect.right() >= cw + 8.0 { "right" }
               else { "bottom" };

    let _ = content.set_attribute("data-rs-side", side);
}
