use leptos::prelude::*;
use super::spinner_ui::{Spinner, SpinnerSize};

#[island]
pub fn SpinnerIsland(
    #[prop(optional, into)] size: Option<String>,
    #[prop(optional)] paused: Option<bool>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let size = match size.as_deref() {
        Some("small") => SpinnerSize::Small,
        Some("large") => SpinnerSize::Large,
        _ => SpinnerSize::Medium,
    };
    let paused = paused.unwrap_or(false);
    let aria_label = aria_label.unwrap_or_else(|| "Loading".to_string());
    let cls = class.unwrap_or_default();

    view! {
        <Spinner size=size paused=paused aria_label=aria_label class=cls />
    }
}
