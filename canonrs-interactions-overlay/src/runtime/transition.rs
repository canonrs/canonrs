//! Transition — duration reader e state lifecycle (entering/open/exiting/closed)

use web_sys::Element;
use super::state;

/// Lê --{prefix}-transition-duration do computed style e retorna ms
pub fn duration_ms(el: &Element, css_var: &str) -> i32 {
    if let Some(win) = web_sys::window() {
        if let Ok(Some(style)) = win.get_computed_style(el) {
            let val = style.get_property_value(css_var).unwrap_or_default();
            let val = val.trim();
            if val.ends_with("ms") {
                if let Ok(n) = val.trim_end_matches("ms").trim().parse::<u32>() {
                    return n as i32;
                }
            } else if val.ends_with('s') {
                if let Ok(n) = val.trim_end_matches('s').trim().parse::<f32>() {
                    return (n * 1000.0) as i32;
                }
            }
        }
    }
    200_i32
}

/// Aplica state em overlay e content
pub fn set_state_nodes(overlay: &Option<Element>, content: &Option<Element>, s: &str) {
    for el in [overlay, content].iter().filter_map(|e| e.as_ref()) {
        match s {
            "entering" | "open" => {
                state::remove_state(el, "closed");
                state::remove_state(el, "exiting");
                state::add_state(el, s);
            }
            "exiting" => {
                state::remove_state(el, "open");
                state::remove_state(el, "entering");
                state::add_state(el, "exiting");
            }
            _ => {
                state::remove_state(el, "open");
                state::remove_state(el, "entering");
                state::remove_state(el, "exiting");
                state::add_state(el, "closed");
            }
        }
    }
}
