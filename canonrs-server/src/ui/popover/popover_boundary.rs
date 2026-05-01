//! @canon-level: strict
//! Popover Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::popover_ui::{
    Popover as PopoverUi,
    PopoverTrigger as PopoverTriggerUi,
    PopoverContent as PopoverContentUi
};
use canonrs_core::meta::VisibilityState;
pub use canonrs_core::primitives::PopoverSide;



#[component]
pub fn Popover(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(into, default = String::new())] name: String,
) -> impl IntoView {
    view! {
        <PopoverUi state=VisibilityState::Closed class=class.unwrap_or_default() name=name>
            {children()}
        </PopoverUi>
    }
}

#[component]
pub fn PopoverContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(default = PopoverSide::Bottom)] side: PopoverSide,
) -> impl IntoView {
    view! { <PopoverContentUi side=side class=class.unwrap_or_default()>{children()}</PopoverContentUi> }
}

#[component]
pub fn PopoverTrigger(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] label: String,
) -> impl IntoView {
    view! { <PopoverTriggerUi class=class.unwrap_or_default() value=value label=label>{children()}</PopoverTriggerUi> }
}
