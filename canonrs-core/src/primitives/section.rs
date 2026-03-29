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
        <div data-rs-section-header="" data-rs-component="SectionHeader" class=class>
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
        <h2 data-rs-section-title="" data-rs-component="SectionTitle" class=class>
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
        <p data-rs-section-subtitle="" data-rs-component="SectionSubtitle" class=class>
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
        <span data-rs-section-badge="" data-rs-component="SectionBadge" class=class>
            {children()}
        </span>
    }
}
