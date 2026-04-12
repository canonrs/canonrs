#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::doc_progress::{DocProgressPrimitive, DocProgressPortal, DocProgressPosition};

pub use canonrs_core::primitives::doc_progress::DocProgressPosition as DocProgressSlotPosition;

#[component]
pub fn DocProgress(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <DocProgressPrimitive class=class progress=0u8 />
    }
}

#[component]
pub fn DocProgressSlot(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] scroll_target: String,
    #[prop(default = DocProgressPosition::Top)] position: DocProgressPosition,
) -> impl IntoView {
    view! {
        <DocProgressPortal class=class scroll_target=scroll_target position=position />
    }
}
