//! Registry — mapa selector → init_fn
//! Tier S: zero if-chains, extensível, isolado

use web_sys::Element;

pub type InitFn = fn(Element);

pub fn dispatch(el: &Element) {
    if el.has_attribute("data-rs-table")      { crate::table::init(el.clone()); return; }
    if el.has_attribute("data-rs-tooltip")    { crate::tooltip::init(el.clone()); return; }
    if el.has_attribute("data-rs-collapsible"){ crate::collapsible::init(el.clone()); return; }
    if el.has_attribute("data-rs-switch")     { crate::switch::init(el.clone()); return; }
    if el.has_attribute("data-rs-toggle")     { crate::toggle::init(el.clone()); return; }
    if el.has_attribute("data-rs-checkbox")   { crate::checkbox::init(el.clone()); return; }
    if el.has_attribute("data-rs-animate")    { crate::animate::init(el.clone()); return; }
    if el.has_attribute("data-rs-avatar-group") { crate::avatar::init(el.clone()); return; }
    if el.has_attribute("data-rs-alert")      { crate::alert::init(el.clone()); return; }
    if el.has_attribute("data-rs-banner")     { crate::banner::init(el.clone()); return; }
    if el.has_attribute("data-rs-button")       { crate::button::init(el.clone()); return; }
    if el.has_attribute("data-rs-doc-progress")   { crate::doc_progress::init(el.clone()); return; }
    if el.has_attribute("data-rs-icon-button")    { crate::icon_button::init(el.clone()); return; }
    if el.has_attribute("data-rs-input-group")      { crate::input_group::init(el.clone()); return; }
    if el.has_attribute("data-rs-input-otp")        { crate::input_otp::init(el.clone()); return; }
    if el.has_attribute("data-rs-menu")             { crate::menu::init(el.clone()); return; }
    if el.has_attribute("data-rs-navigation-menu")  { crate::navigation_menu::init(el.clone()); return; }
    if el.has_attribute("data-rs-toast")            { crate::toast::init(el.clone()); return; }
    if el.has_attribute("data-rs-toc")              { crate::table_of_contents::init(el.clone()); return; }
}

pub fn scan_all() {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
    let selector = "[data-rs-table],[data-rs-tooltip],[data-rs-collapsible],[data-rs-switch],[data-rs-toggle],[data-rs-checkbox],[data-rs-animate],[data-rs-avatar-group],[data-rs-alert],[data-rs-banner],[data-rs-button],[data-rs-doc-progress],[data-rs-icon-button],[data-rs-input-group],[data-rs-input-otp],[data-rs-menu],[data-rs-navigation-menu],[data-rs-toast],[data-rs-toc]";
    let Ok(list) = doc.query_selector_all(selector) else { return };
    for i in 0..list.length() {
        if let Some(el) = list.item(i).and_then(|n| {
            use wasm_bindgen::JsCast;
            n.dyn_into::<Element>().ok()
        }) {
            dispatch(&el);
        }
    }
}
