use leptos::prelude::*;
use canonrs_core::primitives::StatusDotPrimitive;

pub use canonrs_core::primitives::StatusDotVariant;

#[component]
pub fn StatusDot(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = StatusDotVariant::Offline)] variant: StatusDotVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <StatusDotPrimitive variant=variant class=class id=id>
            {children.map(|c| c())}
        </StatusDotPrimitive>
    }
}

#[component]
pub fn StatusDotPreview() -> impl IntoView {
    view! { <StatusDot>"Active"</StatusDot> }
}
