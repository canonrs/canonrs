
use leptos::prelude::*;
use canonrs_core::primitives::{SectionHeaderPrimitive, SectionTitlePrimitive, SectionSubtitlePrimitive, SectionBadgePrimitive};

#[component]
pub fn SectionHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionHeaderPrimitive class=class>{children()}</SectionHeaderPrimitive> }
}

#[component]
pub fn SectionTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionTitlePrimitive class=class>{children()}</SectionTitlePrimitive> }
}

#[component]
pub fn SectionSubtitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionSubtitlePrimitive class=class>{children()}</SectionSubtitlePrimitive> }
}

#[component]
pub fn SectionBadge(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionBadgePrimitive class=class>{children()}</SectionBadgePrimitive> }
}

#[component]
pub fn SectionActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-section-actions="" class=class>
            {children()}
        </div>
    }
}
