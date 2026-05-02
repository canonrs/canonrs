#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{ToolbarPrimitive, ToolbarSeparatorPrimitive, ToolbarOrientation};

#[component]
pub fn Toolbar(
    children: Children,
    #[prop(into, default = String::new())] uid: String,
    #[prop(into)] aria_label: String,
    #[prop(default = ToolbarOrientation::Horizontal)] orientation: ToolbarOrientation,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ToolbarPrimitive
            uid=uid
            class=class
            aria_label=aria_label
            orientation=orientation
        >
            {children()}
        </ToolbarPrimitive>
    }
}

#[component]
pub fn ToolbarSeparator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ToolbarSeparatorPrimitive class=class />
    }
}
