use leptos::prelude::*;
use crate::ui::section::section_ui::{
    SectionHeader as SectionHeaderUi,
    SectionTitle as SectionTitleUi,
    SectionSubtitle as SectionSubtitleUi,
    SectionBadge as SectionBadgeUi,
    SectionActions as SectionActionsUi
};

#[component]
pub fn SectionHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionHeaderUi class=class>{children()}</SectionHeaderUi> }
}

#[component]
pub fn SectionTitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionTitleUi class=class>{children()}</SectionTitleUi> }
}

#[component]
pub fn SectionSubtitle(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionSubtitleUi class=class>{children()}</SectionSubtitleUi> }
}

#[component]
pub fn SectionBadge(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionBadgeUi class=class>{children()}</SectionBadgeUi> }
}

#[component]
pub fn SectionActions(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SectionActionsUi class=class>{children()}</SectionActionsUi> }
}
