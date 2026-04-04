use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ButtonGroupAttached {
    #[default]
    Detached,
    Attached,
}

impl ButtonGroupAttached {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Detached => "",
            Self::Attached => "attached",
        }
    }
}

#[island]
pub fn ButtonGroupIsland(
    children: Children,
    #[prop(optional)] attached: Option<ButtonGroupAttached>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    let attached  = attached.unwrap_or_default();
    let class     = class.unwrap_or_default();

    let state = move || {
        match attached {
            ButtonGroupAttached::Attached => "attached",
            ButtonGroupAttached::Detached => "",
        }
    };

    view! {
        <div
            data-rs-button-group=""
            data-rs-component="ButtonGroup"
            data-rs-behavior="action"
            data-rs-state=state
            role="group"
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}
