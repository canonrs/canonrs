//! @canon-level: ui
//! DropdownMenu - attribute-driven
//! Trigger: attr:data-rs-dropdown-menu-trigger=""

use leptos::prelude::*;
use canonrs_core::separator::SeparatorOrientation;
use canonrs_core::primitives::{
    DropdownMenuPrimitive, DropdownMenuContentPrimitive,
    DropdownMenuGroupPrimitive, DropdownMenuItemPrimitive,
    DropdownMenuCheckboxItemPrimitive,
    LabelPrimitive, SeparatorPrimitive,
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
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-trigger=""
            aria-haspopup="menu"
            aria-expanded="false"
            class=class
            id=id
        >
            {children()}
        </button>
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
        <div data-rs-dropdown-menu-label="" class=class>
            <LabelPrimitive>{children()}</LabelPrimitive>
        </div>
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
            <button type="button" data-rs-dropdown-menu-trigger="">"Options ▼"</button>
            <DropdownMenuContent>
                <DropdownMenuItem>"Item"</DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
