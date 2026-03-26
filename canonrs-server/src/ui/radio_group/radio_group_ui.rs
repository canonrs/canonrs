//! @canon-level: ui
//! RadioGroup - native HTML input, sem behavior

use leptos::prelude::*;
use canonrs_core::primitives::{RadioGroupPrimitive, RadioGroupItemPrimitive};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn RadioGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioGroupPrimitive class=class>
            {children()}
        </RadioGroupPrimitive>
    }
}

#[component]
pub fn RadioGroupItem(
    children: Children,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = false)] checked: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let selected_state = if checked { SelectionState::Selected } else { SelectionState::Unselected };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <RadioGroupItemPrimitive
            selected=selected_state
            disabled=disabled_state
            value=value
            name=name
            class=class
        >
            {children()}
        </RadioGroupItemPrimitive>
    }
}

#[component]
pub fn RadioGroupPreview() -> impl IntoView {
    view! {
        <RadioGroup>
            <RadioGroupItem name="rg-preview" value="a">"Option A"</RadioGroupItem>
            <RadioGroupItem name="rg-preview" value="b">"Option B"</RadioGroupItem>
        </RadioGroup>
    }
}
