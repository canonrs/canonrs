//! @canon-id: command
//! @canon-label: Command
//! @canon-family: interactive
//! @canon-category: Action
//! @canon-intent: Command palette for quick actions
//! @canon-description: Command palette
//! @canon-composable: true
//! @canon-capabilities: OpenClose, Typeahead
//! @canon-required-parts: CommandInput, CommandList
//! @canon-optional-parts: CommandItem, CommandGroup, CommandSeparator, CommandEmpty
//! @canon-tags: command, palette, spotlight, search, shortcut

use leptos::prelude::*;
use canonrs_core::primitives::{
    CommandPrimitive, CommandInputPrimitive, CommandListPrimitive,
    CommandEmptyPrimitive, CommandGroupPrimitive, CommandGroupHeadingPrimitive,
    CommandItemPrimitive, CommandSeparatorPrimitive,
};
use canonrs_core::meta::SelectionState;

#[component]
pub fn Command(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CommandPrimitive class=class>
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
        <CommandInputPrimitive placeholder=placeholder class=class />
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
        <CommandGroupPrimitive class=class>
            {heading.map(|h: String| view! {
                <CommandGroupHeadingPrimitive>{h}</CommandGroupHeadingPrimitive>
            })}
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
    let selected_state = if selected { SelectionState::Selected } else { SelectionState::Unselected };
    view! {
        <CommandItemPrimitive value=value.unwrap_or_default() selected=selected_state class=class>
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
