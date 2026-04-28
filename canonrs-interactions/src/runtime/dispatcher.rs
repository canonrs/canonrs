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

pub fn dispatch(el: &Element) {
    let group = el.get_attribute("data-rs-interaction").unwrap_or_default();
    HANDLERS.with(|h| {
        if let Some(handler) = h.borrow().get(&group) {
            handler(el.clone());
        }
    });
}

pub fn register(group: &str, handler: Handler) {
    HANDLERS.with(|h| {
        h.borrow_mut().insert(group.to_string(), handler);
    });
}