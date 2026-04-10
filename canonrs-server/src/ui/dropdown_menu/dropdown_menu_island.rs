//! @canon-level: strict
//! DropdownMenu Island — passthrough only

use leptos::prelude::*;
use super::dropdown_menu_ui::{
    DropdownMenu, DropdownMenuTrigger, DropdownMenuContent,
    DropdownMenuGroup, DropdownMenuItem, DropdownMenuCheckboxItem,
    DropdownMenuSeparator,
};
use canonrs_core::meta::{DisabledState, ToggleState};

#[component]
pub fn DropdownMenuIsland(
    children: Children,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let label = trigger_label.unwrap_or_else(|| "Options".to_string());
    view! {
        <DropdownMenu class=class.unwrap_or_default()>
            <DropdownMenuTrigger>{label}</DropdownMenuTrigger>
            <DropdownMenuContent>{children()}</DropdownMenuContent>
        </DropdownMenu>
    }
}

#[component]
pub fn DropdownMenuTriggerIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuTrigger class=class>{children()}</DropdownMenuTrigger> }
}

#[component]
pub fn DropdownMenuContentIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuContent class=class>{children()}</DropdownMenuContent> }
}

#[component]
pub fn DropdownMenuGroupIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuGroup class=class>{children()}</DropdownMenuGroup> }
}

#[component]
pub fn DropdownMenuItemIsland(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuItem disabled=disabled class=class>{children()}</DropdownMenuItem> }
}

#[component]
pub fn DropdownMenuCheckboxItemIsland(
    children: Children,
    #[prop(default = ToggleState::Off)] checked: ToggleState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuCheckboxItem checked=checked disabled=disabled class=class>{children()}</DropdownMenuCheckboxItem> }
}

#[component]
pub fn DropdownMenuSeparatorIsland() -> impl IntoView {
    view! { <DropdownMenuSeparator /> }
}
