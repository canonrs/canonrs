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
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(default = ToolbarOrientation::Horizontal)] orientation: ToolbarOrientation,
) -> impl IntoView {
    view! {
        <div
            data-rs-toolbar=""
            data-rs-uid=crate::infra::uid::generate("tb2")
            data-rs-interaction="nav"
            data-rs-component="Toolbar"
            data-rs-behavior="actions"
            data-rs-variant=orientation.as_str()
            role="toolbar"
            aria-label=aria_label
            aria-orientation=orientation.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ToolbarSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
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
