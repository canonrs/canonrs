use web_sys::Element;

pub fn dispatch(el: &Element) {
    let group = el.get_attribute("data-rs-interaction").unwrap_or_default();
    match group.as_str() {
        "init"      => canonrs_interactions_init::init_init(el.clone()),
        "nav"       => canonrs_interactions_nav::init_nav(el.clone()),
        "data"      => canonrs_interactions_data::init_data(el.clone()),
        "gesture"   => canonrs_interactions_gesture::init_gesture(el.clone()),
        "overlay"   => canonrs_interactions_overlay::init_overlay(el.clone()),
        "selection" => canonrs_interactions_selection::init_selection(el.clone()),
        "content"   => canonrs_interactions_content::init_content(el.clone()),
        _ => {}
    }
}