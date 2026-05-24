//! Filter Interaction Engine

use wasm_bindgen::JsCast;
use web_sys::Element;
use canonrs_interactions_core::dom::query;
use canonrs_interactions_core::runtime::listeners;

fn apply_filter(root: &Element) {
    let search = query::first(root, "[data-rs-filter-input]")
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|el| el.value().to_lowercase()).unwrap_or_default();
    let severity = query::all(root, "[data-rs-filter-select]").iter()
        .find(|el| el.get_attribute("data-rs-filter-key").as_deref() == Some("severity"))
        .and_then(|el| el.get_attribute("data-rs-selected-value")).unwrap_or_default().to_lowercase();
    let category = query::all(root, "[data-rs-filter-select]").iter()
        .find(|el| el.get_attribute("data-rs-filter-key").as_deref() == Some("category"))
        .and_then(|el| el.get_attribute("data-rs-selected-value")).unwrap_or_default().to_lowercase();
    let doc = match web_sys::window().and_then(|w| w.document()) { Some(d) => d, None => return };
    let target_id = match root.get_attribute("data-rs-filter-target") { Some(id) => id, None => return };
    let target = match doc.get_element_by_id(&target_id) { Some(el) => el, None => return };
    let rows = query::all(&target, "[data-rs-table-row]");
    let mut visible = 0usize;
    for row in &rows {
        let text = row.text_content().unwrap_or_default().to_lowercase();
        let row_sev = row.get_attribute("data-rs-severity").unwrap_or_default().to_lowercase();
        let row_cat = row.get_attribute("data-rs-category").unwrap_or_default().to_lowercase();
        if (search.is_empty() || text.contains(&search)) &&
           (severity.is_empty() || row_sev == severity) &&
           (category.is_empty() || row_cat == category) {
            let _ = row.class_list().remove_1("row--hidden"); visible += 1;
        } else {
            let _ = row.class_list().add_1("row--hidden");
        }
    }
    if let Some(counter) = query::first(root, "[data-rs-count]") {
        counter.set_text_content(Some(&format!("{} rules", visible)));
    }
}

pub fn init(root: Element) {
    let uid = root.get_attribute("data-rs-uid").unwrap_or_default();
    for input in query::all(&root, "[data-rs-filter-input]") {
        let uid_i = format!("{}:input", uid);
        let root_c = root.clone();
        listeners::listen(&uid_i, &input, "input", move |_: web_sys::Event| { apply_filter(&root_c); });
    }
    listeners::listen(&uid, &root, "rs-change", {
        let root_c = root.clone();
        move |e: web_sys::Event| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
            let Some(select) = target.closest("[data-rs-filter-select]").ok().flatten() else { return };
            let value = target.get_attribute("data-rs-value").unwrap_or_default();
            let _ = select.set_attribute("data-rs-selected-value", &value);
            apply_filter(&root_c);
        }
    });
}
