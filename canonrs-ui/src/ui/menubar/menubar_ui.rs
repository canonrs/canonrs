use leptos::prelude::*;
use crate::primitives::{
    MenubarPrimitive,
    MenubarTriggerPrimitive,
    MenubarContentPrimitive,
    MenubarSubItemPrimitive,
    MenubarSeparatorPrimitive,
};

#[component]
pub fn Menubar(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <MenubarPrimitive
            class=class
            id=id
        >
            {children()}
        </MenubarPrimitive>
    }
}

#[component]
pub fn MenubarTrigger(
    children: Children,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <MenubarTriggerPrimitive
            controls_id=controls_id
            expanded=expanded
            class=class
            id=id
        >
            {children()}
        </MenubarTriggerPrimitive>
    }
}

#[component]
pub fn MenubarContent(
    children: Children,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenubarContentPrimitive
            content_id=content_id
            class=class
        >
            {children()}
        </MenubarContentPrimitive>
    }
}

#[component]
pub fn MenubarSubItem(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <MenubarSubItemPrimitive
            class=class
            id=id
        >
            {children()}
        </MenubarSubItemPrimitive>
    }
}

#[component]
pub fn MenubarSeparator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenubarSeparatorPrimitive
            class=class
        />
    }
}
