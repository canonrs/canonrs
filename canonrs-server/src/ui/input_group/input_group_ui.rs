//! @canon-id: input-group
//! @canon-label: Input Group
//! @canon-family: utility
//! @canon-category: Form
//! @canon-intent: Group input with prefix or suffix elements
//! @canon-description: Input group with addons
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: input-group, prefix, suffix, addon, field

use leptos::prelude::*;
use canonrs_core::primitives::InputGroupPrimitive;

#[component]
pub fn InputGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] merge_radius: bool,
) -> impl IntoView {
    view! {
        <InputGroupPrimitive class=class merge_radius=merge_radius>
            {children()}
        </InputGroupPrimitive>
    }
}

#[component]
pub fn InputGroupPreview() -> impl IntoView {
    view! {
        <InputGroup merge_radius=true>
            <span data-rs-input-group-addon="">"@"</span>
        </InputGroup>
    }
}
