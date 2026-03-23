use leptos::prelude::*;
use canonrs_core::primitives::{
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
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <PopoverTriggerPrimitive
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

#[component]
pub fn PopoverPreview() -> impl IntoView {
    view! { <Popover id="popover-preview".to_string()>"Content"</Popover> }
}
