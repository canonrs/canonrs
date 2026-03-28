//! @canon-id: toggle-group
//! @canon-label: Toggle Group
//! @canon-family: interactive
//! @canon-category: Action
//! @canon-intent: Group of toggle buttons with single or multiple selection
//! @canon-description: Group of toggle buttons
//! @canon-composable: true
//! @canon-capabilities: Multiple, Disabled
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: toggle-group, toggle, group, buttons, options, selection

use leptos::prelude::*;
use canonrs_core::primitives::ToggleGroupPrimitive;

#[component]
pub fn ToggleGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] multiple: bool,
) -> impl IntoView {
    view! {
        <ToggleGroupPrimitive class=class multiple=multiple>
            {children()}
        </ToggleGroupPrimitive>
    }
}
