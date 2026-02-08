use leptos::prelude::*;
use crate::primitives::{
    MenuPrimitive,
    MenuItemPrimitive,
    MenuGroupPrimitive,
    LabelPrimitive,
    SeparatorPrimitive,
};

#[component]
pub fn Menu(
    children: Children,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <MenuPrimitive 
            aria_label={aria_label.unwrap_or_default()} 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
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
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <MenuItemPrimitive
            disabled={disabled}
            selected={selected}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            {children()}
        </MenuItemPrimitive>
    }
}

#[component]
pub fn MenuGroup(
    children: Children,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <MenuGroupPrimitive 
            aria_label={aria_label.unwrap_or_default()} 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </MenuGroupPrimitive>
    }
}

#[component]
pub fn MenuLabel(
    children: Children,
    #[prop(optional)] _html_for: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            attr:data-menu-label=""
            role="presentation"
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            <LabelPrimitive>
                {children()}
            </LabelPrimitive>
        </div>
    }
}

#[component]
pub fn MenuSeparator(
    #[prop(default = "horizontal".to_string())] orientation: String,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <SeparatorPrimitive
            orientation={orientation}
            decorative={true}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        />
    }
}
