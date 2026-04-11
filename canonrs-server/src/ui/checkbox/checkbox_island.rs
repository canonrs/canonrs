//! @canon-level: strict
//! Checkbox Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::checkbox_ui::Checkbox;
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn CheckboxIsland(
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let checked_state = if checked { ActivityState::Active } else { ActivityState::Inactive };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <Checkbox checked=checked_state disabled=disabled_state name=name class=class>
            {children()}
        </Checkbox>
    }
}
