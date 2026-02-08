use leptos::prelude::*;
use crate::shared::StatusVariant;
use crate::primitives::StatusDotPrimitive;

#[component]
pub fn StatusDot(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = StatusVariant::Idle)] variant: StatusVariant,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <StatusDotPrimitive
            variant={variant.as_str().to_string()}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </StatusDotPrimitive>
    }
}
