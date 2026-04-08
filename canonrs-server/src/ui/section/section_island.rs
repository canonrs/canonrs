use leptos::prelude::*;
use crate::ui::section::section_ui::{SectionHeader, SectionTitle, SectionSubtitle, SectionBadge, SectionActions};

#[component]
pub fn SectionHeaderIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionHeader class=class>{children()}</SectionHeader> }
}

#[component]
pub fn SectionTitleIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionTitle class=class>{children()}</SectionTitle> }
}

#[component]
pub fn SectionSubtitleIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionSubtitle class=class>{children()}</SectionSubtitle> }
}

#[component]
pub fn SectionBadgeIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionBadge class=class>{children()}</SectionBadge> }
}

#[component]
pub fn SectionActionsIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionActions class=class>{children()}</SectionActions> }
}
