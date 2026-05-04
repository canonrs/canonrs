//! @canon-level: strict
//! @canon-owner: primitives-team
//! Spinner Primitive - Loading indicator

use leptos::prelude::*;
use crate::meta::LoadingState;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SpinnerSize {
    Small,
    #[default]
    Medium,
    Large,
}
impl SpinnerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small  => "small",
            Self::Medium => "medium",
            Self::Large  => "large",
        }
    }
}

#[component]
pub fn SpinnerPrimitive(
    #[prop(default = SpinnerSize::Medium)] size: SpinnerSize,
    #[prop(default = LoadingState::Loading)] state: LoadingState,
    #[prop(into, default = "Loading".to_string())] aria_label: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_spn = crate::infra::uid::generate("spn");
    view! {
        <svg
            data-rs-spinner=""
            data-rs-uid=uid_spn
            data-rs-size=size.as_str()
            data-rs-loading=state.as_str()
            role="status"
            aria-label=aria_label
            aria-busy=state.aria_busy()
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
        >
            <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
    }
}
