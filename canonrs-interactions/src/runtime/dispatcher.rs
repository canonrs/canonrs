use web_sys::Element;
use std::cell::RefCell;
use std::collections::HashMap;

type Handler = fn(Element);

thread_local! {
    static HANDLERS: RefCell<HashMap<String, Handler>> = RefCell::new({
        let mut m: HashMap<String, Handler> = HashMap::new();
        m.insert("init".into(),      canonrs_interactions_init::init_init);
        m.insert("nav".into(),       canonrs_interactions_nav::init_nav);
        m.insert("data".into(),      canonrs_interactions_data::init_data);
        m.insert("gesture".into(),   canonrs_interactions_gesture::init_gesture);
        m.insert("overlay".into(),   canonrs_interactions_overlay::init_overlay);
        m.insert("selection".into(), canonrs_interactions_selection::init_selection);
        m.insert("content".into(),   canonrs_interactions_content::init_content);
        m
    });
}

/// Bootstrap kernel — propagate data-rs-owner to children before component init
/// This is bootstrap responsibility, NOT component responsibility
fn propagate_owner(root: &Element, uid: &str) {
    use wasm_bindgen::JsCast;
    let Ok(nodes) = root.query_selector_all("[data-rs-uid], [data-rs-interaction], [data-rs-state]") else { return };
    for i in 0..nodes.length() {
        if let Some(el) = nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
            if el.get_attribute("data-rs-uid").is_none() {
                let _ = el.set_attribute("data-rs-owner", uid);
            }
        }
    }
}

pub fn dispatch(el: &Element) {
    let group = el.get_attribute("data-rs-interaction").unwrap_or_default();
    // Register in ownership tree + set lifecycle Mount
    if let Some(uid) = el.get_attribute("data-rs-uid") {
        canonrs_interactions_core::runtime::ownership::register(&uid, None);
        canonrs_interactions_core::runtime::lifecycle::set_state(&uid, canonrs_interactions_core::runtime::lifecycle::LifecycleState::Mount);
    }
    // Bootstrap responsibility: propagate ownership BEFORE component init
    if let Some(uid) = el.get_attribute("data-rs-uid") {
        propagate_owner(&el, &uid);
    }

    HANDLERS.with(|h| {
        if let Some(handler) = h.borrow().get(&group) {
            handler(el.clone());
            // Mark as initialized so observer ignores future mutations in this subtree
            let _ = el.set_attribute("data-rs-initialized", "true");
        }
    });
    // Transition to Active after dispatch
    if let Some(uid) = el.get_attribute("data-rs-uid") {
        canonrs_interactions_core::runtime::lifecycle::set_state(&uid, canonrs_interactions_core::runtime::lifecycle::LifecycleState::Active);
    }
}

pub fn register(group: &str, handler: Handler) {
    HANDLERS.with(|h| {
        h.borrow_mut().insert(group.to_string(), handler);
    });
}

/// Exposto via wasm_bindgen para plugins externos registrarem handlers
/// Ex: rs-canvas-runtime chama canonrs_interactions::register_interaction("canvas", fn)
pub fn register_external(group: String, handler: Handler) {
    HANDLERS.with(|h| {
        h.borrow_mut().insert(group, handler);
    });
}