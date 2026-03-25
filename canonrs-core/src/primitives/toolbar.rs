//! @canon-level: strict
//! @canon-owner: primitives-team
//! Toolbar Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ToolbarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ToolbarOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical   => "vertical",
        }
    }
}

#[component]
pub fn ToolbarPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] aria_label: String,
    #[prop(default = ToolbarOrientation::Horizontal)] orientation: ToolbarOrientation,
) -> impl IntoView {
    view! {
        <div
            data-rs-toolbar=""
            role="toolbar"
            aria-label=aria_label
            aria-orientation=orientation.as_str()
            class=class
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ToolbarSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-toolbar-separator=""
            role="separator"
            aria-orientation="vertical"
            class=class
        />
    }
}
