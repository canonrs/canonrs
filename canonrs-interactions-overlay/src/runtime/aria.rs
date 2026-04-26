//! Aria — linkage automático de labelledby e describedby

use web_sys::Element;

/// Liga aria-labelledby e aria-describedby no content a partir do uid
pub fn link_aria(
    uid: &str,
    content: &Element,
    title_attr: &str,
    desc_attr: &str,
) {
    if let Some(portal_parent) = content.parent_element() {
        if let Some(title) = portal_parent.query_selector(&format!("[{}]", title_attr)).ok().flatten() {
            let id = format!("{}-title", uid);
            let _ = title.set_attribute("id", &id);
            let _ = content.set_attribute("aria-labelledby", &id);
        }
        if let Some(desc) = portal_parent.query_selector(&format!("[{}]", desc_attr)).ok().flatten() {
            let id = format!("{}-desc", uid);
            let _ = desc.set_attribute("id", &id);
            let _ = content.set_attribute("aria-describedby", &id);
        }
    }
}

/// Liga a partir do portal diretamente
pub fn link_aria_from_portal(
    uid: &str,
    portal: &Element,
    content_attr: &str,
    title_attr: &str,
    desc_attr: &str,
) {
    let content = portal.query_selector(&format!("[{}]", content_attr)).ok().flatten();
    let title   = portal.query_selector(&format!("[{}]", title_attr)).ok().flatten();
    let desc    = portal.query_selector(&format!("[{}]", desc_attr)).ok().flatten();

    if let Some(ref content) = content {
        if let Some(title) = title {
            let id = format!("{}-title", uid);
            let _ = title.set_attribute("id", &id);
            let _ = content.set_attribute("aria-labelledby", &id);
        }
        if let Some(desc) = desc {
            let id = format!("{}-desc", uid);
            let _ = desc.set_attribute("id", &id);
            let _ = content.set_attribute("aria-describedby", &id);
        }
    }
}
