//! @canon-level: strict
//! DocProgress Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::doc_progress_ui::{
    DocProgress as DocProgressUi,
    DocProgressSlot as DocProgressSlotUi,
    DocProgressSlotPosition
};

#[component]
pub fn DocProgress(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <DocProgressUi class=class /> }
}

#[component]
pub fn DocProgressSlot(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] scroll_target: String,
    #[prop(into, default = String::from("top"))] position: String,
) -> impl IntoView {
    let pos = match position.as_str() {
        "bottom" => DocProgressSlotPosition::Bottom,
        _        => DocProgressSlotPosition::Top,
    };
    view! { <DocProgressSlotUi class=class scroll_target=scroll_target position=pos /> }
}
