use leptos::prelude::*;
use crate::primitives::{
    MenuPrimitive,
    MenuItemPrimitive,
    MenuGroupPrimitive,
    MenuLabelPrimitive,
    MenuSeparatorPrimitive,
};

#[component]
pub fn Menu(
    children: Children,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <MenuPrimitive
            aria_label={aria_label.unwrap_or_default()}
            class={class}
            id={id}
        >
            {children()}
        </MenuPrimitive>
    }
}

#[component]
pub fn MenuItem(
    children: Children,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] selected: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <MenuItemPrimitive
            disabled={disabled}
            selected={selected}
            class={class}
            id={id}
        >
            {children()}
        </MenuItemPrimitive>
    }
}

#[component]
pub fn MenuGroup(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenuGroupPrimitive
            class={class}
        >
            {children()}
        </MenuGroupPrimitive>
    }
}

#[component]
pub fn MenuLabel(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenuLabelPrimitive
            class={class}
        >
            {children()}
        </MenuLabelPrimitive>
    }
}

#[component]
pub fn MenuSeparator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenuSeparatorPrimitive
            class={class}
        />
    }
}
