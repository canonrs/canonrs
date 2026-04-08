//! ButtonGroup Island — Canon Rule #340 + #341
//! Passthrough + DOM-driven state. Zero signals.

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ButtonGroupAttached {
    #[default] Detached,
    Attached,
}

impl ButtonGroupAttached {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Attached => "attached", Self::Detached => "" }
    }
}

#[component]
pub fn ButtonGroupIsland(
    children: Children,
    #[prop(default = ButtonGroupAttached::Detached)] attached: ButtonGroupAttached,
    #[prop(into, default = String::new())] class:              String,
    #[prop(optional, into)] aria_label:                        Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-button-group=""
            data-rs-component="ButtonGroup"
            data-rs-behavior="action"
            data-rs-state=attached.as_str()
            role="group"
            aria-label=aria_label
            class=class
        >
            {children()}
        </div>
    }
}
