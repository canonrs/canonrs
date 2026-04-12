//! canonrs-interactions-nav

pub mod runtime;
pub mod sidebar;
pub mod menubar;
pub mod toolbar;
pub mod breadcrumb;
pub mod link_group;
pub mod pagination;
pub mod tabs;
pub mod accordion;

pub fn init_nav(el: web_sys::Element) {
    if el.has_attribute("data-rs-sidebar")   { sidebar::init(el.clone()); }
    if el.has_attribute("data-rs-tabs")      { tabs::init(el.clone()); }
    if el.has_attribute("data-rs-accordion") { accordion::init(el.clone()); }
    if el.has_attribute("data-rs-menubar") { menubar::init(el.clone()); }
    if el.has_attribute("data-rs-toolbar") { toolbar::init(el.clone()); }
    if el.has_attribute("data-rs-breadcrumb") { breadcrumb::init(el.clone()); }
    if el.has_attribute("data-rs-link-group") { link_group::init(el.clone()); }
    if el.has_attribute("data-rs-pagination") { pagination::init(el.clone()); }
}
