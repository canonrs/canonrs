#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::StatusDotPrimitive;
pub use canonrs_core::primitives::StatusDotVariant;

#[component]
pub fn StatusDot(
    children: Children,
    #[prop(default = StatusDotVariant::Offline)] variant: StatusDotVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <StatusDotPrimitive variant=variant class=class>
            {children()}
        </StatusDotPrimitive>
    }
}

