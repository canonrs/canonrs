//! @canon-id: select
//! @canon-label: Select
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Choose one option from a list
//! @canon-description: Dropdown select input
//! @canon-composable: true
//! @canon-capabilities: OpenClose, Disabled
//! @canon-required-parts: SelectTrigger, SelectContent, SelectItem
//! @canon-optional-parts: SelectValue, SelectSeparator
//! @canon-tags: select, dropdown, choose, options, list, combo

use leptos::prelude::*;
use canonrs_core::meta::{SelectionState, DisabledState};
use canonrs_core::primitives::{
    SelectPrimitive, SelectTriggerPrimitive, SelectValuePrimitive,
    SelectContentPrimitive, SelectItemPrimitive, SelectSeparatorPrimitive,
};

#[component]
pub fn Select(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectPrimitive class=class node_ref=node_ref.unwrap_or_default()>
            {children()}
        </SelectPrimitive>
    }
}

#[component]
pub fn SelectTrigger(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectTriggerPrimitive disabled=disabled class=class>
            {children()}
        </SelectTriggerPrimitive>
    }
}

#[component]
pub fn SelectValue(
    children: Children,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectValuePrimitive placeholder=placeholder class=class>
            {children()}
        </SelectValuePrimitive>
    }
}

#[component]
pub fn SelectContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectContentPrimitive class=class>
            {children()}
        </SelectContentPrimitive>
    }
}

#[component]
pub fn SelectItem(
    children: Children,
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectItemPrimitive selected=selected disabled=disabled value=value class=class>
            {children()}
        </SelectItemPrimitive>
    }
}

#[component]
pub fn SelectSeparator(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SelectSeparatorPrimitive class=class /> }
}

#[component]
pub fn SelectPreview() -> impl IntoView {
    use canonrs_core::meta::DisabledState;
    view! {
        <Select>
            <SelectTrigger>
                <SelectValue>"Select an option..."</SelectValue>
            </SelectTrigger>
            <SelectContent>
                <SelectItem value="a">"Option A"</SelectItem>
                <SelectItem value="b">"Option B"</SelectItem>
                <SelectSeparator />
                <SelectItem value="c" disabled=DisabledState::Disabled>"Disabled"</SelectItem>
            </SelectContent>
        </Select>
    }
}
