use leptos::prelude::*;
use crate::primitives::{ToolbarPrimitive, ToolbarSeparatorPrimitive};

#[derive(Clone, Copy, PartialEq)]
pub enum ToolbarOrientation {
    Horizontal,
    Vertical,
}

impl ToolbarOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[component]
pub fn Toolbar(
    #[prop(optional)] children: Option<Children>,
    #[prop(into)] aria_label: String, // OBRIGATÓRIO com into
    #[prop(default = ToolbarOrientation::Horizontal)] orientation: ToolbarOrientation,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ToolbarPrimitive
            class={class}
            id={id.unwrap_or_default()}
            aria_label={aria_label}
            orientation={orientation.as_str().to_string()}
        >
            {children.map(|c| c())}
        </ToolbarPrimitive>
    }
}

#[component]
pub fn ToolbarSeparator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ToolbarSeparatorPrimitive class={class} />
    }
}
