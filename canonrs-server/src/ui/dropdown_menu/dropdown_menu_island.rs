//! @canon-level: strict
//! DropdownMenu Island — passthrough only

use leptos::prelude::*;
use super::dropdown_menu_ui::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem};
use canonrs_core::meta::DisabledState;

#[component]
pub fn DropdownMenuIsland(
    children: Children,
    #[prop(optional, into)] trigger_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <DropdownMenu class=class.unwrap_or_default()>
            <DropdownMenuTrigger>{trigger_label.unwrap_or_else(|| "Options".to_string())}</DropdownMenuTrigger>
            <DropdownMenuContent>{children()}</DropdownMenuContent>
        </DropdownMenu>
    }
}

#[component]
pub fn DropdownMenuItemIsland(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! { <DropdownMenuItem disabled=disabled>{children()}</DropdownMenuItem> }
}
