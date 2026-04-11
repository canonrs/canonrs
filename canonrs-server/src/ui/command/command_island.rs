//! @canon-level: strict
//! Command Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::command_ui::{Command, CommandInput, CommandList, CommandItem};

#[component]
pub fn CommandIsland(
    children: Children,
    #[prop(into, default = String::from("Search..."))] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Command class=class>
            <CommandInput placeholder=placeholder />
            <CommandList>{children()}</CommandList>
        </Command>
    }
}

#[allow(unused_variables)]
#[component]
pub fn CommandItemIsland(
    children: Children,
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, optional)] group: Option<String>,
) -> impl IntoView {
    view! {
        <CommandItem value=value.unwrap_or_default()>
            {children()}
        </CommandItem>
    }
}
