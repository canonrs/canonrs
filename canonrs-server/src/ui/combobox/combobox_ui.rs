//! @canon-level: ui
//! Combobox - attribute-driven
//! Relação trigger↔list via estrutura DOM

use leptos::prelude::*;
use canonrs_core::primitives::{
    ComboboxPrimitive,
    ComboboxTriggerPrimitive,
    ComboboxListPrimitive,
    ComboboxItemPrimitive,
};

#[component]
pub fn Combobox(
    children: Children,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ComboboxPrimitive
            expanded={expanded}
            disabled={disabled}
            class={class}
            id={id.unwrap_or_default()}
        >
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
    view! {
        <ComboboxTriggerPrimitive disabled={disabled} class={class}>
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
        <ComboboxListPrimitive class={class}>
            {children()}
        </ComboboxListPrimitive>
    }
}

#[component]
pub fn ComboboxItem(
    children: Children,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ComboboxItemPrimitive selected={selected} disabled={disabled} class={class}>
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
