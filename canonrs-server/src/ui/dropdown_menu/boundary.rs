//! @canon-level: strict
//! DropdownMenu Island — passthrough only

use leptos::prelude::*;
use super::dropdown_menu_ui::{
    DropdownMenu as DropdownMenuUi,
    DropdownMenuTrigger as DropdownMenuTriggerUi,
    DropdownMenuContent as DropdownMenuContentUi,
    DropdownMenuGroup as DropdownMenuGroupUi,
    DropdownMenuItem as DropdownMenuItemUi,
    DropdownMenuCheckboxItem as DropdownMenuCheckboxItemUi,
    DropdownMenuSeparator as DropdownMenuSeparatorUi
};
use canonrs_core::meta::{DisabledState, ToggleState};

#[component]
pub fn DropdownMenu(
    children: Children,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DropdownMenuUi class=class.unwrap_or_default()>
            <DropdownMenuTriggerUi>{trigger_label.unwrap_or_else(|| "Options".to_string())}</DropdownMenuTriggerUi>
            <DropdownMenuContentUi>{children()}</DropdownMenuContentUi>
        </DropdownMenuUi>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuTriggerUi class=class>{children()}</DropdownMenuTriggerUi> }
}

#[component]
pub fn DropdownMenuContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuContentUi class=class>{children()}</DropdownMenuContentUi> }
}

#[component]
pub fn DropdownMenuGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuGroupUi class=class>{children()}</DropdownMenuGroupUi> }
}

#[component]
pub fn DropdownMenuItem(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuItemUi disabled=disabled class=class>{children()}</DropdownMenuItemUi> }
}

#[component]
pub fn DropdownMenuCheckboxItem(
    children: Children,
    #[prop(default = ToggleState::Off)] checked: ToggleState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DropdownMenuCheckboxItemUi checked=checked disabled=disabled class=class>{children()}</DropdownMenuCheckboxItemUi> }
}

#[component]
pub fn DropdownMenuSeparator() -> impl IntoView {
    view! { <DropdownMenuSeparatorUi /> }
}
