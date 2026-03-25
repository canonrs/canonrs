//! @canon-level: ui
//! Command - attribute-driven
//! Filtro via behavior que lê data-rs-command-input e oculta data-rs-command-item

use leptos::prelude::*;
use canonrs_core::primitives::{
    CommandPrimitive,
    CommandInputPrimitive,
    CommandListPrimitive,
    CommandEmptyPrimitive,
    CommandGroupPrimitive,
    CommandItemPrimitive,
    CommandSeparatorPrimitive,
};

#[component]
pub fn Command(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <CommandPrimitive class=class id=id.unwrap_or_default()>
            {children()}
        </CommandPrimitive>
    }
}

#[component]
pub fn CommandInput(
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CommandInputPrimitive
            placeholder=placeholder
            class=class
        />
    }
}

#[component]
pub fn CommandList(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CommandListPrimitive class=class>
            {children()}
        </CommandListPrimitive>
    }
}

#[component]
pub fn CommandEmpty(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CommandEmptyPrimitive class=class>
            {children()}
        </CommandEmptyPrimitive>
    }
}

#[component]
pub fn CommandGroup(
    children: Children,
    #[prop(optional, into)] heading: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CommandGroupPrimitive heading=heading.unwrap_or_default() class=class>
            {children()}
        </CommandGroupPrimitive>
    }
}

#[component]
pub fn CommandItem(
    children: Children,
    #[prop(optional, into)] value: Option<String>,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CommandItemPrimitive value=value.unwrap_or_default() selected=selected class=class>
            {children()}
        </CommandItemPrimitive>
    }
}

#[component]
pub fn CommandSeparator(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CommandSeparatorPrimitive class=class />
    }
}

#[component]
pub fn CommandPreview() -> impl IntoView {
    view! {
        <Command>
            <CommandInput placeholder="Search..." />
            <CommandList>
                <CommandGroup heading="Suggestions">
                    <CommandItem>"Calendar"</CommandItem>
                    <CommandItem>"Search"</CommandItem>
                    <CommandItem>"Settings"</CommandItem>
                </CommandGroup>
            </CommandList>
        </Command>
    }
}
