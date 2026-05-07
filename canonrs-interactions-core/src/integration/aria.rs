//! Aria — helpers para atributos ARIA
use web_sys::Element;

pub fn set_expanded(el: &Element, expanded: bool) {
    let _ = el.set_attribute("aria-expanded", if expanded { "true" } else { "false" });
}

pub fn set_selected(el: &Element, selected: bool) {
    let _ = el.set_attribute("aria-selected", if selected { "true" } else { "false" });
}

pub fn set_hidden(el: &Element, hidden: bool) {
    let _ = el.set_attribute("aria-hidden", if hidden { "true" } else { "false" });
}

pub fn set_disabled(el: &Element, disabled: bool) {
    let _ = el.set_attribute("aria-disabled", if disabled { "true" } else { "false" });
}

pub fn set_pressed(el: &Element, pressed: bool) {
    let _ = el.set_attribute("aria-pressed", if pressed { "true" } else { "false" });
}

pub fn set_checked(el: &Element, checked: bool) {
    let _ = el.set_attribute("aria-checked", if checked { "true" } else { "false" });
}

/// Liga aria-labelledby e aria-describedby no content a partir do uid
pub fn link_aria(uid: &str, content: &Element, title_attr: &str, desc_attr: &str) {
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
    portal: &web_sys::Element,
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
