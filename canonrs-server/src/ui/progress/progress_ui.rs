//! @canon-id: progress
//! @canon-label: Progress
//! @canon-family: feedback
//! @canon-category: Feedback
//! @canon-intent: Show completion of a task
//! @canon-description: Progress bar indicator
//! @canon-composable: true
//! @canon-capabilities: Value
//! @canon-required-parts: ProgressIndicator
//! @canon-optional-parts:
//! @canon-tags: progress, bar, loading, percentage, completion

use leptos::prelude::*;
use canonrs_core::primitives::{ProgressPrimitive, ProgressIndicatorPrimitive};

#[component]
pub fn Progress(
    #[prop(default = 0.0)] value: f64,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let clamped = value.clamp(0.0, 100.0);
    let style = format!("transform: translateX(-{}%)", 100.0 - clamped);
    view! {
        <ProgressPrimitive value=clamped class={class.unwrap_or_default()}>
            <ProgressIndicatorPrimitive style=style />
        </ProgressPrimitive>
    }
}

#[component]
pub fn ProgressPreview() -> impl IntoView {
    view! {
        <Progress value=50.0 />
    }
}
