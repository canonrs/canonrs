//! CanonRS Runtime — detector e router de grupos de interação.

pub mod detect;
pub mod init;

const EXTERNAL_GROUPS: &[&str] = &["gesture", "overlay", "selection", "nav", "data", "content"];

/// Ponto de entrada único. Chamado após hydrate_islands().
#[cfg(target_arch = "wasm32")]
pub fn init() {
    let groups = detect::detect_groups();

    for group in &groups {
        if EXTERNAL_GROUPS.contains(&group.as_str()) {
            load_group(group);
        }
    }

    init::init_all(groups);
}

#[cfg(target_arch = "wasm32")]
fn load_group(group: &str) {
    use wasm_bindgen::JsCast;
    use js_sys::{Function, Reflect};
    use wasm_bindgen::JsValue;

    let win = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let loader = match Reflect::get(&win, &JsValue::from_str("__canonLoader")) {
        Ok(v) => v,
        Err(_) => return,
    };

    let func = match Reflect::get(&loader, &JsValue::from_str("loadGroup")) {
        Ok(v) => v,
        Err(_) => return,
    };

    if let Ok(f) = func.dyn_into::<Function>() {
        let _ = f.call1(&loader, &JsValue::from_str(group));
    }
}
