use crate::primitives::command::*;
use leptos::prelude::*;
use crate::primitives::{
    CommandEmptyPrimitive, CommandGroupPrimitive, CommandItemPrimitive, SeparatorPrimitive,
};

#[component]
pub fn Command(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CommandPrimitive
            attr:class={class_name}
            id={id}
        >
            {children()}
        </CommandPrimitive>
    }
}

#[component]
pub fn CommandInputSimple(
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CommandInputPrimitive
            placeholder={placeholder}
            value=value
            attr:class={class_name}
            id={id}
        >
            ""
        </CommandInputPrimitive>
    }
}

#[component]
pub fn CommandListSimple(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CommandListPrimitive
            attr:class={class_name}
            id={id}
        >
            {children()}
        </CommandListPrimitive>
    }
}

#[component]
pub fn CommandEmpty(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CommandEmptyPrimitive
            attr:class={class_name}
            id={id}
        >
            {children()}
        </CommandEmptyPrimitive>
    }
}

#[component]
pub fn CommandGroup(
    children: Children,
    #[prop(optional)] heading: Option<String>,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CommandGroupPrimitive
            heading={heading.unwrap_or_default()}
            attr:class={class_name}
            id={id}
        >
            {children()}
        </CommandGroupPrimitive>
    }
}

#[component]
pub fn CommandItemSimple(
    children: Children,
    #[prop(optional)] value: Option<String>,
    #[prop(default = false)] selected: bool,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <CommandItemPrimitive
            value={value.unwrap_or_default()}
            selected=selected
            attr:class={class_name}
            id={id}
        >
            {children()}
        </CommandItemPrimitive>
    }
}

#[component]
pub fn CommandSeparator(
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <SeparatorPrimitive
            orientation="horizontal".to_string()
            decorative=true
            attr:class={class_name}
            id={id}
        />
    }
}
