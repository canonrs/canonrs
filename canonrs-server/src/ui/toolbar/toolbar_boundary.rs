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
    #[prop(into, default = String::new())] uid: String,
    #[prop(into, default = String::from("Toolbar"))] aria_label: String,
    #[prop(default = ToolbarOrientation::Horizontal)] orientation: ToolbarOrientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ToolbarUi uid=uid aria_label=aria_label orientation=orientation class=class>
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
        <div
            data-rs-toolbar-item=""
            data-rs-value=value
            tabindex="-1"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ToolbarSeparator() -> impl IntoView {
    view! { <ToolbarSeparatorUi /> }
}
