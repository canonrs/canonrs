//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn IconChevronDown(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="m6 9 6 6 6-6"/>
        </svg>
    }
}

#[component]
pub fn IconMenu(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="4" x2="20" y1="12" y2="12"/><line x1="4" x2="20" y1="6" y2="6"/><line x1="4" x2="20" y1="18" y2="18"/>
        </svg>
    }
}

#[component]
pub fn IconBadgeCheck(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="m9 12 2 2 4-4"/>
        </svg>
    }
}

#[component]
pub fn IconBell(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"/><path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"/>
        </svg>
    }
}

#[component]
pub fn IconCheck(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M20 6 9 17l-5-5"/>
        </svg>
    }
}

#[component]
pub fn IconChevronsUpDown(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>
        </svg>
    }
}

#[component]
pub fn IconCreditCard(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect width="20" height="14" x="2" y="5" rx="2"/><line x1="2" x2="22" y1="10" y2="10"/>
        </svg>
    }
}

#[component]
pub fn IconLogOut(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" x2="9" y1="12" y2="12"/>
        </svg>
    }
}

#[component]
pub fn IconTrendingUp(#[prop(default = "".to_string())] class: String) -> impl IntoView {
    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="22 7 13.5 15.5 8.5 10.5 2 17"/><polyline points="16 7 22 7 22 13"/>
        </svg>
    }
}
