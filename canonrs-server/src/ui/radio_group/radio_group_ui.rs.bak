#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{RadioGroupPrimitive, RadioGroupItemPrimitive};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn RadioGroup(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioGroupPrimitive disabled=disabled class=class>
            {children()}
        </RadioGroupPrimitive>
    }
}

#[component]
pub fn RadioGroupItem(
    children: Children,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioGroupItemPrimitive
            selected=selected
            disabled=disabled
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
