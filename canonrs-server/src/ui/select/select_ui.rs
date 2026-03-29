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
use canonrs_core::meta::SelectionState;
use canonrs_core::primitives::{
    SelectPrimitive, SelectTriggerPrimitive, SelectValuePrimitive,
    SelectContentPrimitive, SelectItemPrimitive, SelectSeparatorPrimitive,
};

#[component]
pub fn Select(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectPrimitive class=class>
            {children()}
        </SelectPrimitive>
    }
}

#[component]
pub fn SelectTrigger(
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectTriggerPrimitive disabled=disabled.into() class=class>
            {children()}
        </SelectTriggerPrimitive>
    }
}

#[component]
pub fn SelectValue(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectValuePrimitive placeholder=placeholder class=class>
            {children.map(|c| c())}
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
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SelectItemPrimitive selected=selected.into() disabled=disabled.into() value=value class=class>
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
    view! {
        <Select>
            <SelectTrigger>
                <SelectValue placeholder="Select an option..." />
            </SelectTrigger>
            <SelectContent>
                <SelectItem value="a">"Option A"</SelectItem>
                <SelectItem value="b">"Option B"</SelectItem>
                <SelectSeparator />
                <SelectItem value="c" disabled=true>"Disabled"</SelectItem>
            </SelectContent>
        </Select>
    }
}
