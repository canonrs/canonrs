use leptos::prelude::*;
use canonrs_core::primitives::FieldPrimitive;

#[island]
pub fn FieldIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let class    = class.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);

    view! {
        <FieldPrimitive
            attr:data-rs-orientation="vertical"
            attr:data-rs-validation="none"
            attr:data-rs-disabled=if disabled { "true" } else { "false" }
            class=class
        >
            {children()}
        </FieldPrimitive>
    }
}
