//! @canon-level: strict
//! @canon-owner: primitives-team
//! Section UI Primitives - HTML puro

use leptos::prelude::*;

#[component]
pub fn SectionHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-section-header="" data-rs-uid=crate::infra::uid::generate("sec") class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SectionTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h2 data-rs-section-title="" class=class>
            {children()}
        </h2>
    }
}

#[component]
pub fn SectionSubtitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-section-subtitle="" class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn SectionBadgePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-section-badge="" class=class>
            {children()}
        </span>
    }
}
