//! @canon-level: strict
//! @canon-owner: primitives-team
//! Spinner Primitive - Loading indicator

use leptos::prelude::*;

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
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[component]
pub fn SpinnerPrimitive(
    #[prop(default = SpinnerSize::Medium)] size: SpinnerSize,
    #[prop(default = false)] paused: bool,
    #[prop(into, default = "Loading".to_string())] aria_label: String,
    #[prop(into, optional)] style: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <svg
            data-spinner=""
            data-size={size.as_str()}
            data-paused={if paused { "true" } else { "false" }}
            role="status"
            aria-label=aria_label
            aria-busy="true"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            style=style
            class=class
            id=id
        >
            <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
    }
}
