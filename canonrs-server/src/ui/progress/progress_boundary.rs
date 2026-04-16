//! @canon-level: strict
//! Progress Boundary — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::progress_ui::Progress as ProgressUi;
use canonrs_core::primitives::progress::ProgressState;

#[component]
pub fn Progress(
    #[prop(default = 0.0)] value: f64,
    #[prop(default = ProgressState::Default)] state: ProgressState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <ProgressUi value=value state=state class=class /> }
}
