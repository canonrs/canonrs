use leptos::prelude::*;
use crate::primitives::{
    DropdownMenuPrimitive,
    DropdownMenuTriggerPrimitive,
    DropdownMenuContentPrimitive,
    DropdownMenuGroupPrimitive,
    DropdownMenuItemPrimitive,
    DropdownMenuCheckboxItemPrimitive,
    LabelPrimitive,
    SeparatorPrimitive,
};

#[component]
pub fn DropdownMenu(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DropdownMenuPrimitive
            class=class
            id=id
        >
            {children()}
        </DropdownMenuPrimitive>
    }
}

#[component]
pub fn DropdownMenuTrigger(
    children: Children,
    #[prop(into)] target_dropdown_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DropdownMenuTriggerPrimitive
            target_dropdown_id=target_dropdown_id
            class=class
            id=id
        >
            {children()}
        </DropdownMenuTriggerPrimitive>
    }
}

#[component]
pub fn DropdownMenuContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DropdownMenuContentPrimitive
            class=class
            id=id
        >
            {children()}
        </DropdownMenuContentPrimitive>
    }
}

#[component]
pub fn DropdownMenuGroup(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DropdownMenuGroupPrimitive
            class=class
            id=id
        >
            {children()}
        </DropdownMenuGroupPrimitive>
    }
}

#[component]
pub fn DropdownMenuItem(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DropdownMenuItemPrimitive
            class=class
            id=id
        >
            {children()}
        </DropdownMenuItemPrimitive>
    }
}

#[component]
pub fn DropdownMenuCheckboxItem(
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <DropdownMenuCheckboxItemPrimitive
            checked=checked
            class=class
            id=id
        >
            {children()}
        </DropdownMenuCheckboxItemPrimitive>
    }
}

#[component]
pub fn DropdownMenuLabel(
    children: Children,
    #[prop(default = String::new())] html_for: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-dropdown-menu-label=""
            class=class
            id=id
        >
            <LabelPrimitive>
                {children()}
            </LabelPrimitive>
        </div>
    }
}

#[component]
pub fn DropdownMenuSeparator(
    #[prop(default = "horizontal".to_string())] orientation: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SeparatorPrimitive
            orientation=orientation
            decorative=true
            class=class
            id=id
        />
    }
}
