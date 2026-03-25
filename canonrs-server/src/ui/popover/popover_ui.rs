//! @canon-level: ui
//! Popover - attribute-driven
//! Trigger: attr:data-rs-popover-trigger=""

use leptos::prelude::*;
use canonrs_core::primitives::{PopoverPrimitive, PopoverContentPrimitive};

#[component]
pub fn Popover(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PopoverPrimitive open=open.into() class=class>
            {children()}
        </PopoverPrimitive>
    }
}

#[component]
pub fn PopoverContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PopoverContentPrimitive class=class>
            {children()}
        </PopoverContentPrimitive>
    }
}

#[component]
pub fn PopoverPreview() -> impl IntoView {
    view! {
        <Popover>
            <button type="button" data-rs-popover-trigger="">"Open Popover"</button>
            <PopoverContent>"Popover content"</PopoverContent>
        </Popover>
    }
}
