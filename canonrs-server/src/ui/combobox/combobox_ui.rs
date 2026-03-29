//! @canon-id: combobox
//! @canon-label: Combobox
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Search and select from a list
//! @canon-description: Searchable combo box
//! @canon-composable: true
//! @canon-capabilities: OpenClose, Disabled
//! @canon-required-parts: ComboboxTrigger, ComboboxList, ComboboxItem
//! @canon-optional-parts:
//! @canon-tags: combobox, search, autocomplete, filter, combo

use leptos::prelude::*;
use canonrs_core::primitives::{
    ComboboxPrimitive, ComboboxTriggerPrimitive,
    ComboboxListPrimitive, ComboboxItemPrimitive,
};
use canonrs_core::meta::{DisabledState, SelectionState};

#[component]
pub fn Combobox(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxPrimitive class=class node_ref=node_ref.unwrap_or_default()>
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
            <ComboboxTrigger>"Select option..."</ComboboxTrigger>
            <ComboboxList>
                <ComboboxItem value="1">"Option 1"</ComboboxItem>
                <ComboboxItem value="2">"Option 2"</ComboboxItem>
            </ComboboxList>
        </Combobox>
    }
}
