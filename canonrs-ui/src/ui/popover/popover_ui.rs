use leptos::prelude::*;
use crate::primitives::{
    PopoverPrimitive,
    PopoverTriggerPrimitive,
    PopoverContentPrimitive,
};

#[component]
pub fn Popover(
    children: Children,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PopoverPrimitive
            open=open
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
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PopoverTriggerPrimitive
            controls_id=controls_id
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
    #[prop(default = String::new())] content_id: String,
    #[prop(default = "center".to_string())] align: String,
    #[prop(default = 4)] side_offset: i32,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PopoverContentPrimitive
            content_id=content_id
            align=align
            side_offset=side_offset
            class=class
        >
            {children()}
        </PopoverContentPrimitive>
    }
}
