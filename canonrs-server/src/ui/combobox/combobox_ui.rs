//! @canon-id: combobox
//! @canon-label: Combobox
//! @canon-family: input
//! @canon-category: Form
//! @canon-intent: Search and select from a list
//! @canon-description: Searchable combo box
//! @canon-composable: true
//! @canon-capabilities: OpenClose, Disabled
//! @canon-required-parts: SelectTrigger, SelectContent
//! @canon-optional-parts:
//! @canon-tags: combobox, search, autocomplete, filter, combo

use leptos::prelude::*;
use canonrs_core::primitives::{
    ComboboxPrimitive, ComboboxTriggerPrimitive,
    ComboboxListPrimitive, ComboboxItemPrimitive,
};
use canonrs_core::meta::{VisibilityState, DisabledState, SelectionState};

#[component]
pub fn Combobox(
    children: Children,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] _disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if expanded { VisibilityState::Open } else { VisibilityState::Closed };
    view! {
        <ComboboxPrimitive state=state class=class>
            {children()}
        </ComboboxPrimitive>
    }
}

#[component]
pub fn ComboboxTrigger(
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <ComboboxTriggerPrimitive disabled=disabled_state class=class>
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
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let selected_state = if selected { SelectionState::Selected } else { SelectionState::Unselected };
    let disabled_state = if disabled { DisabledState::Disabled } else { DisabledState::Enabled };
    view! {
        <ComboboxItemPrimitive selected=selected_state disabled=disabled_state value=value class=class>
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
                <ComboboxItem>"Option 1"</ComboboxItem>
                <ComboboxItem>"Option 2"</ComboboxItem>
            </ComboboxList>
        </Combobox>
    }
}
