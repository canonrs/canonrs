use leptos::prelude::*;
use crate::primitives::StatusDotPrimitive;

pub use crate::primitives::StatusDotVariant;

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
