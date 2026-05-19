#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::separator::SeparatorOrientation;
use canonrs_core::primitives::{
    DropdownMenuPrimitive, DropdownMenuContentPrimitive,
    DropdownMenuGroupPrimitive, DropdownMenuItemPrimitive,
    DropdownMenuCheckboxItemPrimitive, DropdownMenuTriggerPrimitive,
    DropdownMenuLabelPrimitive,
    SeparatorPrimitive,
};
use canonrs_core::meta::{DisabledState, ToggleState, VisibilityState};

#[component]
pub fn DropdownMenu(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuPrimitive state=VisibilityState::Closed class=class node_ref=node_ref.unwrap_or_default()>
            {children()}
        </DropdownMenuPrimitive>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    children: Children,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuTriggerPrimitive disabled=disabled class=class>
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
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuItemPrimitive disabled=disabled class=class>
            {children()}
        </DropdownMenuItemPrimitive>
    }
}

#[component]
pub fn DropdownMenuCheckboxItem(
    children: Children,
    #[prop(default = ToggleState::Off)] checked: ToggleState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DropdownMenuCheckboxItemPrimitive checked=checked disabled=disabled class=class>
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
