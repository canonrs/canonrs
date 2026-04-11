//! @canon-level: strict
//! Menu Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::menu_ui::{Menu, MenuItem};
use canonrs_core::meta::{DisabledState, SelectionState};

#[component]
pub fn MenuIsland(
    children: Children,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <Menu aria_label=aria_label class=class>{children()}</Menu> }
}

#[component]
pub fn MenuItemIsland(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let selected_state = if selected { SelectionState::Selected } else { SelectionState::Unselected };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    let _ = value; // stored in data-rs-value by primitive
    view! { <MenuItem selected=selected_state disabled=disabled_state class=class>{children()}</MenuItem> }
}
