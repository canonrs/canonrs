
use leptos::prelude::*;
use canonrs_core::primitives::{
    EmptyStatePrimitive, EmptyStateIconPrimitive,
    EmptyStateTitlePrimitive, EmptyStateDescriptionPrimitive,
    EmptyStateActionPrimitive,
};
pub use canonrs_core::primitives::EmptyStateVariant;

#[component]
pub fn EmptyState(
    children: Children,
    #[prop(default = EmptyStateVariant::Default)] variant: EmptyStateVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <EmptyStatePrimitive variant=variant class=class>
            {children()}
        </EmptyStatePrimitive>
    }
}

#[component]
pub fn EmptyStateIcon(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <EmptyStateIconPrimitive class=class>
            {children()}
        </EmptyStateIconPrimitive>
    }
}

#[component]
pub fn EmptyStateTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <EmptyStateTitlePrimitive class=class>
            {children()}
        </EmptyStateTitlePrimitive>
    }
}

#[component]
pub fn EmptyStateDescription(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <EmptyStateDescriptionPrimitive class=class>
            {children()}
        </EmptyStateDescriptionPrimitive>
    }
}

#[component]
pub fn EmptyStateAction(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <EmptyStateActionPrimitive class=class>
            {children()}
        </EmptyStateActionPrimitive>
    }
}

#[component]
pub fn EmptyStatePreview() -> impl IntoView {
    view! {
        <EmptyState>
            <EmptyStateTitle>"No items found"</EmptyStateTitle>
            <EmptyStateDescription>"Try adjusting your search or filters."</EmptyStateDescription>
        </EmptyState>
    }
}
