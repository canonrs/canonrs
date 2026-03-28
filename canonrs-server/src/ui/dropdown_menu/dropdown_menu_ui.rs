//! @canon-id: dropdown-menu
//! @canon-label: Dropdown Menu
//! @canon-family: interactive
//! @canon-category: Action
//! @canon-intent: Show contextual action menu
//! @canon-description: Dropdown menu
//! @canon-composable: true
//! @canon-capabilities: OpenClose, Disabled
//! @canon-required-parts: DropdownMenuTrigger, DropdownMenuContent
//! @canon-optional-parts: DropdownMenuItem, DropdownMenuSeparator, DropdownMenuGroup
//! @canon-tags: dropdown-menu, dropdown, menu, options, actions

use leptos::prelude::*;
use canonrs_core::separator::SeparatorOrientation;
use canonrs_core::primitives::{
    DropdownMenuPrimitive, DropdownMenuContentPrimitive,
    DropdownMenuGroupPrimitive, DropdownMenuItemPrimitive,
    DropdownMenuCheckboxItemPrimitive, DropdownMenuTriggerPrimitive,
    DropdownMenuLabelPrimitive, SeparatorPrimitive,
};
use canonrs_core::meta::ToggleState;

#[component]
pub fn DropdownMenu(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuPrimitive class=class>
            {children()}
        </DropdownMenuPrimitive>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuTriggerPrimitive class=class>
            {children()}
        </DropdownMenuTriggerPrimitive>
    }
}

#[component]
pub fn DropdownMenuContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuContentPrimitive class=class>
            {children()}
        </DropdownMenuContentPrimitive>
    }
}

#[component]
pub fn DropdownMenuGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuGroupPrimitive class=class>
            {children()}
        </DropdownMenuGroupPrimitive>
    }
}

#[component]
pub fn DropdownMenuItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuItemPrimitive class=class>
            {children()}
        </DropdownMenuItemPrimitive>
    }
}

#[component]
pub fn DropdownMenuCheckboxItem(
    children: Children,
    #[prop(default = ToggleState::Off)] checked: ToggleState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuCheckboxItemPrimitive checked=checked class=class>
            {children()}
        </DropdownMenuCheckboxItemPrimitive>
    }
}

#[component]
pub fn DropdownMenuLabel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuLabelPrimitive class=class>
            {children()}
        </DropdownMenuLabelPrimitive>
    }
}

#[component]
pub fn DropdownMenuSeparator(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SeparatorPrimitive orientation=SeparatorOrientation::Horizontal decorative=true class=class />
    }
}

#[component]
pub fn DropdownMenuPreview() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>"Options ▼"</DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuItem>"Item"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
