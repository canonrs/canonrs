//! @canon-level: strict
//! InputGroup Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::input_group_ui::InputGroup;
use canonrs_core::meta::ToggleState;

#[component]
pub fn InputGroupIsland(
    children: Children,
    #[prop(default = false)] merge_radius: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let merge = if merge_radius { ToggleState::On } else { ToggleState::Off };
    view! {
        <InputGroup merge_radius=merge class=class>
            {children()}
        </InputGroup>
    }
}
