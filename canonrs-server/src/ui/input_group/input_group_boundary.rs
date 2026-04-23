//! @canon-level: strict
//! InputGroup Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::input_group_ui::{InputGroup as InputGroupUi, InputGroupPrefix as InputGroupPrefixUi, InputGroupSuffix as InputGroupSuffixUi};
use canonrs_core::meta::ToggleState;

#[component]
pub fn InputGroup(
    children: Children,
    #[prop(default = false)] merge_radius: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let merge = if merge_radius { ToggleState::On
} else { ToggleState::Off };
    view! {
        <InputGroupUi merge_radius=merge class=class>
            {children()}
        </InputGroupUi>
    }
}

#[component]
pub fn InputGroupPrefix(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InputGroupPrefixUi class=class>
            {children()}
        </InputGroupPrefixUi>
    }
}

#[component]
pub fn InputGroupSuffix(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InputGroupSuffixUi class=class>
            {children()}
        </InputGroupSuffixUi>
    }
}
