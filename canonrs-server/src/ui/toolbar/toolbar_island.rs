//! Toolbar Island — Canon Rule passthrough
use leptos::prelude::*;
use super::toolbar_ui::{Toolbar, ToolbarSeparator};
use canonrs_core::primitives::ToolbarOrientation;

#[component]
pub fn ToolbarIsland(
    children: Children,
    #[prop(into, default = String::from("Toolbar"))] aria_label: String,
    #[prop(default = ToolbarOrientation::Horizontal)] orientation: ToolbarOrientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Toolbar aria_label=aria_label orientation=orientation class=class>
            {children()}
        </Toolbar>
    }
}

#[component]
pub fn ToolbarItemIsland(
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
pub fn ToolbarSeparatorIsland() -> impl IntoView {
    view! { <ToolbarSeparator /> }
}
