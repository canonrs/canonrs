//! @canon-level: strict
//! Command Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::command_ui::{
    CommandInput,
    CommandList,
    CommandItem as CommandItemUi
};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Command(
    children: Children,
    #[prop(into, default = String::from("Search..."))] placeholder: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <super::command_ui::Command class=class state=VisibilityState::Open>
            <CommandInput placeholder=placeholder />
            <CommandList>{children()}</CommandList>
        </super::command_ui::Command>
    }
}

#[allow(unused_variables)]
#[component]
pub fn CommandItem(
    children: Children,
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, optional)] group: Option<String>,
) -> impl IntoView {
    view! {
        <CommandItemUi value=value.unwrap_or_default()>
            {children()}
        </CommandItemUi>
    }
}
