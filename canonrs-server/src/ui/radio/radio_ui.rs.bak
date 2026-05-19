#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{RadioPrimitive, RadioGroupPrimitive, RadioGroupItemPrimitive};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn Radio(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioPrimitive
            selected=selected
            disabled=disabled
            value=value
            name=name
            class=class
        >
            {children()}
        </RadioPrimitive>
    }
}

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
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
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
pub fn RadioPreview() -> impl IntoView {
    view! {
        <Radio value="opt1" name="radio-preview">"Option 1"</Radio>
        <Radio value="opt2" name="radio-preview">"Option 2"</Radio>
    }
}
