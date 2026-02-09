use leptos::prelude::*;
use crate::primitives::{
    PopoverPrimitive,
    PopoverTriggerPrimitive,
    PopoverContentPrimitive,
};

#[component]
pub fn Popover(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PopoverPrimitive
            class=class
            id=id
        >
            {children()}
        </PopoverPrimitive>
    }
}

#[component]
pub fn PopoverTrigger(
    children: Children,
    #[prop(into)] target_popover_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PopoverTriggerPrimitive
            target_popover_id=target_popover_id
            class=class
            id=id
        >
            {children()}
        </PopoverTriggerPrimitive>
    }
}

#[component]
pub fn PopoverContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PopoverContentPrimitive
            class=class
            id=id
        >
            {children()}
        </PopoverContentPrimitive>
    }
}
