#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    ComboboxPrimitive, ComboboxInputPrimitive,
    ComboboxListPrimitive, ComboboxItemPrimitive,
};
use canonrs_core::meta::{DisabledState, SelectionState};

#[component]
pub fn Combobox(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] name: String,
) -> impl IntoView {
    view! {
        <ComboboxPrimitive disabled=disabled class=class name=name node_ref=node_ref.unwrap_or_default()>
            {children()}
        </ComboboxPrimitive>
    }
}

#[component]
pub fn ComboboxInput(
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxInputPrimitive placeholder=placeholder disabled=disabled class=class />
    }
}

#[component]
pub fn ComboboxList(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxListPrimitive class=class>
            {children()}
        </ComboboxListPrimitive>
    }
}

#[component]
pub fn ComboboxItem(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxItemPrimitive selected=selected disabled=disabled value=value class=class>
            {children()}
        </ComboboxItemPrimitive>
    }
}

#[component]
pub fn ComboboxPreview() -> impl IntoView {
    view! {
        <Combobox>
            <ComboboxInput placeholder="Search..." />
            <ComboboxList>
                <ComboboxItem value="1">"Option 1"</ComboboxItem>
                <ComboboxItem value="2">"Option 2"</ComboboxItem>
            </ComboboxList>
        </Combobox>
    }
}
