//! @canon-level: ui
//! Popover - attribute-driven

use leptos::prelude::*;
use canonrs_core::primitives::{PopoverPrimitive, PopoverContentPrimitive};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Popover(
    children: Children,
    #[prop(default = false)] open: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let state = if open { VisibilityState::Open } else { VisibilityState::Closed };
    view! {
        <PopoverPrimitive state=state class=class>
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
