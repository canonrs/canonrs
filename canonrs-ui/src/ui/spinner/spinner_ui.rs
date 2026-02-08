use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerSize {
    Small,
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
pub fn Spinner(
    #[prop(default = SpinnerSize::Medium)] size: SpinnerSize,
    #[prop(default = String::new())] custom_size: String,
    #[prop(default = "Loading".to_string())] aria_label: String,
    #[prop(default = false)] paused: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let base_class = format!("spinner size-{} {}", size.as_str(), class);
    let style = if custom_size.is_empty() {
        String::new()
    } else {
        format!("width: {}; height: {};", custom_size, custom_size)
    };

    view! {
        <svg
            data-spinner=""
            attr:data-size={size.as_str()}
            attr:data-paused={if paused { "true" } else { "" }}
            style={style}
            role="status"
            attr:aria-label={aria_label}
            attr:aria-live="polite"
            attr:aria-busy="true"
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class={base_class}
            id={id}
        >
            <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
    }
}
