//! @canon-level: strict
//! Popover Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::popover_ui::{Popover, PopoverContent};
use canonrs_core::meta::VisibilityState;

#[island]
pub fn PopoverInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::popover::init_all();
        });
    }
    view! { <></> }
}

#[component]
pub fn PopoverIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <PopoverInit />
        <Popover state=VisibilityState::Closed class=class.unwrap_or_default()>
            {children()}
        </Popover>
    }
}

#[component]
pub fn PopoverContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <PopoverContent class=class.unwrap_or_default()>{children()}</PopoverContent> }
}
