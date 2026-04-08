//! @canon-level: strict
//! Radio Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::radio_ui::{Radio, RadioGroup, RadioGroupItem};
use canonrs_core::meta::{SelectionState, DisabledState};

#[island]
pub fn RadioInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
        });
    }
    view! { <></> }
}

#[component]
pub fn RadioGroupIsland(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioInit />
        <RadioGroup disabled=disabled class=class>{children()}</RadioGroup>
    }
}

#[component]
pub fn RadioIsland(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Radio selected=selected disabled=disabled value=value name=name class=class>
            {children()}
        </Radio>
    }
}

#[component]
pub fn RadioGroupItemIsland(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <RadioGroupItem selected=selected disabled=disabled value=value name=name class=class>
            {children()}
        </RadioGroupItem>
    }
}
