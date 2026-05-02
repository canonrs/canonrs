//! @canon-level: strict
//! @canon-owner: primitives-team
//! Progress Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ProgressState {
    #[default]
    Default,
    Indeterminate,
    Loading,
}

impl ProgressState {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            ProgressState::Default => None,
            ProgressState::Indeterminate => Some("indeterminate"),
            ProgressState::Loading => Some("loading"),
        }
    }
}

#[component]
pub fn ProgressPrimitive(
    children: Children,
    #[prop(default = 0.0)] value: f64,
    #[prop(default = ProgressState::Default)] state: ProgressState,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_pr = crate::infra::uid::generate("pr");
    let clamped = value.clamp(0.0, 100.0);
    let aria_now = if state == ProgressState::Indeterminate { None } else { Some(clamped.to_string()) };
    view! {
        <div
            data-rs-progress=""
            data-rs-uid=uid_pr
            data-rs-interaction="init"
            data-rs-value=clamped.to_string()
            data-rs-state=state.as_str()
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=aria_now
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ProgressIndicatorPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-progress-indicator=""
            class=class
            style=style
        />
    }
}
