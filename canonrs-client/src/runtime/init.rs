//! Roteia inicialização por grupo de interação.

use wasm_bindgen::JsCast;
use web_sys::Element;

pub fn init_all(groups: Vec<String>) {
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    let nodes = match doc.query_selector_all("[data-rs-interaction]") {
        Ok(n) => n,
        Err(_) => return,
    };

    for i in 0..nodes.length() {
        let el = match nodes.item(i).and_then(|n| n.dyn_into::<Element>().ok()) {
            Some(e) => e,
            None => continue,
        };

        let group = el.get_attribute("data-rs-interaction").unwrap_or_default();

        if !groups.contains(&group) { continue; }

        match group.as_str() {
            #[cfg(feature = "ix_gesture")]
            "gesture" => {
                if el.has_attribute("data-rs-resizable")   { crate::interactions::resizable::init(el.clone()); }
                if el.has_attribute("data-rs-slider")      { crate::interactions::slider::init(el.clone()); }
                if el.has_attribute("data-rs-carousel")    { crate::interactions::carousel::init(el.clone()); }
                if el.has_attribute("data-rs-scroll-area") { crate::interactions::scroll_area::init(el.clone()); }
            }
            #[cfg(feature = "ix_overlay")]
            "overlay" => {
                if el.has_attribute("data-rs-modal")           { crate::interactions::modal::init(el.clone()); }
                if el.has_attribute("data-rs-drawer")          { crate::interactions::drawer::init(el.clone()); }
                if el.has_attribute("data-rs-sheet")           { crate::interactions::sheet::init(el.clone()); }
                if el.has_attribute("data-rs-alert-dialog")    { crate::interactions::alert_dialog::init(el.clone()); }
                if el.has_attribute("data-rs-dialog")          { crate::interactions::dialog::init(el.clone()); }
                if el.has_attribute("data-rs-confirm-dialog")  { crate::interactions::confirm_dialog::init(el.clone()); }
                if el.has_attribute("data-rs-popover")         { crate::interactions::popover::init(el.clone()); }
                if el.has_attribute("data-rs-hover-card")      { crate::interactions::hover_card::init(el.clone()); }
                if el.has_attribute("data-rs-context-menu")    { crate::interactions::context_menu::init(el.clone()); }
                if el.has_attribute("data-rs-dropdown-menu")   { crate::interactions::dropdown_menu::init(el.clone()); }
            }
            #[cfg(feature = "ix_selection")]
            "selection" => {
                if el.has_attribute("data-rs-select")       { crate::interactions::select::init(el.clone()); }
                if el.has_attribute("data-rs-combobox")     { crate::interactions::combobox::init(el.clone()); }
                if el.has_attribute("data-rs-color-picker") { crate::interactions::color_picker::init(el.clone()); }
                if el.has_attribute("data-rs-radio")        { crate::interactions::radio::init(el.clone()); }
                if el.has_attribute("data-rs-toggle-group") { crate::interactions::toggle_group::init(el.clone()); }
                if el.has_attribute("data-rs-tree")         { crate::interactions::tree::init(el.clone()); }
            }
            #[cfg(feature = "ix_nav")]
            "nav" => {
                if el.has_attribute("data-rs-sidebar")     { crate::interactions::sidebar::init(el.clone()); }
                if el.has_attribute("data-rs-menubar")     { crate::interactions::menubar::init(el.clone()); }
                if el.has_attribute("data-rs-toolbar")     { crate::interactions::toolbar::init(el.clone()); }
                if el.has_attribute("data-rs-breadcrumb")  { crate::interactions::breadcrumb::init(el.clone()); }
                if el.has_attribute("data-rs-link-group")  { crate::interactions::link_group::init(el.clone()); }
                if el.has_attribute("data-rs-pagination")  { crate::interactions::pagination::init(el.clone()); }
            }
            #[cfg(feature = "ix_data")]
            "data" => {
                if el.has_attribute("data-rs-data-table")   { crate::interactions::data_table::init(el.clone()); }
                if el.has_attribute("data-rs-table")        { crate::interactions::table::init(el.clone()); }
                if el.has_attribute("data-rs-virtual-list") { crate::interactions::virtual_list::init(el.clone()); }
                if el.has_attribute("data-rs-list-item")    { crate::interactions::list_item::init(el.clone()); }
                if el.has_attribute("data-rs-chart")        { crate::interactions::chart::init(el.clone()); }
            }
            #[cfg(feature = "ix_content")]
            "content" => {
                if el.has_attribute("data-rs-markdown")    { crate::interactions::markdown::init(el.clone()); }
                if el.has_attribute("data-rs-copy-button") { crate::interactions::copy_button::init(el.clone()); }
            }
            _ => {}
        }
    }
}
