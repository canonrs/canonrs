
use leptos::prelude::*;
use canonrs_core::primitives::{
    ComboboxPrimitive, ComboboxTriggerPrimitive,
    ComboboxListPrimitive, ComboboxItemPrimitive,
};
use canonrs_core::meta::{DisabledState, SelectionState, VisibilityState};

#[component]
pub fn Combobox(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxPrimitive state=VisibilityState::Closed class=class node_ref=node_ref.unwrap_or_default()>
            {children()}
        </ComboboxPrimitive>
    }
}

#[component]
pub fn ComboboxTrigger(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxTriggerPrimitive disabled=disabled class=class>
            {children()}
        </ComboboxTriggerPrimitive>
    }
}

#[component]
pub fn ComboboxList(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxListPrimitive state=VisibilityState::Closed class=class>
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
            <ComboboxTrigger>"Select option..."</ComboboxTrigger>
            <ComboboxList>
                <ComboboxItem value="1">"Option 1"</ComboboxItem>
                <ComboboxItem value="2">"Option 2"</ComboboxItem>
            </ComboboxList>
        </Combobox>
    }
}
