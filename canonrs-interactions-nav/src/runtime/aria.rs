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
