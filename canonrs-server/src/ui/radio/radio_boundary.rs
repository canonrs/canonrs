//! @canon-level: strict
//! Radio Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::radio_ui::RadioGroup as RadioGroupUi;
use canonrs_core::meta::{
    SelectionState,
    DisabledState
};



#[component]
pub fn RadioGroup(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioGroupUi disabled=disabled class=class>{children()}</RadioGroupUi>
    }
}

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
        <super::radio_ui::Radio selected=selected disabled=disabled value=value name=name class=class>
            {children()}
        </super::radio_ui::Radio>
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
        <super::radio_ui::RadioGroupItem selected=selected disabled=disabled value=value name=name class=class>
            {children()}
        </super::radio_ui::RadioGroupItem>
    }
}
