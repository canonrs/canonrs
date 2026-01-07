use crate::primitives::popover::{PopoverPrimitive, PopoverTriggerPrimitive, PopoverContentPrimitive};
use leptos::prelude::*;

#[component]
pub fn Popover(
    #[prop(default = false)] open: bool,
    #[prop(default = Callback::new(|_| {}))] on_open_change: Callback<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PopoverPrimitive open=open on_change=on_open_change>
            {children()}
        </PopoverPrimitive>
    }
}

#[component]
pub fn PopoverTrigger(
    #[prop(default = String::new(), into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <PopoverTriggerPrimitive class=class>
            {children()}
        </PopoverTriggerPrimitive>
    }
}

#[component]
pub fn PopoverContent(
    #[prop(default = "center".to_string(), into)] align: String,
    #[prop(default = 4)] side_offset: i32,
    #[prop(default = String::new(), into)] class: String,
    children: ChildrenFn,
) -> impl IntoView {
    let base = "z-50 w-72 rounded-md border bg-popover text-popover-foreground p-4 shadow-md";
    let full = format!("{} {}", base, class);
    view! {
        <PopoverContentPrimitive align=align side_offset=side_offset class=full>
            {children()}
        </PopoverContentPrimitive>
    }
}
