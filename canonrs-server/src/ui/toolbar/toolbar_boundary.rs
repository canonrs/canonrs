//! Toolbar Island — Canon Rule passthrough
use leptos::prelude::*;
use super::toolbar_ui::{
    Toolbar as ToolbarUi,
    ToolbarSeparator as ToolbarSeparatorUi
};
pub use canonrs_core::primitives::ToolbarOrientation;

#[component]
pub fn Toolbar(
    children: Children,
    #[prop(into, default = String::from("Toolbar"))] aria_label: String,
    #[prop(default = ToolbarOrientation::Horizontal)] orientation: ToolbarOrientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ToolbarUi aria_label=aria_label orientation=orientation class=class>
            {children()}
        </ToolbarUi>
    }
}

#[component]
pub fn ToolbarItem(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-toolbar-item=""
            data-rs-value=value
            disabled=disabled
            tabindex="-1"
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ToolbarSeparator() -> impl IntoView {
    view! { <ToolbarSeparatorUi /> }
}
