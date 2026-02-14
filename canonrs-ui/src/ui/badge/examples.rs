use leptos::prelude::*;
use super::badge_ui::{Badge, BadgeVariant};

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display:flex;gap:0.5rem;flex-wrap:wrap;align-items:center;">
            <Badge variant=BadgeVariant::Default>"Default"</Badge>
            <Badge variant=BadgeVariant::Primary>"Primary"</Badge>
            <Badge variant=BadgeVariant::Success>"Success"</Badge>
            <Badge variant=BadgeVariant::Warning>"Warning"</Badge>
            <Badge variant=BadgeVariant::Error>"Error"</Badge>
            <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
            <Badge variant=BadgeVariant::Primary interactive=true>"Interactive"</Badge>
        </div>
    }
}

#[component]
pub fn BasicExample() -> impl IntoView {
    basic_example()
}
