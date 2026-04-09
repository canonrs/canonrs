
use leptos::prelude::*;
use canonrs_core::primitives::{PopoverPrimitive, PopoverTriggerPrimitive, PopoverContentPrimitive};
use canonrs_core::meta::VisibilityState;

#[component]
pub fn Popover(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <PopoverPrimitive state=state class=class>
            {children()}
        </PopoverPrimitive>
    }
}

#[component]
pub fn PopoverTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <PopoverTriggerPrimitive class=class>{children()}</PopoverTriggerPrimitive> }
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
