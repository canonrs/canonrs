//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[component]
pub fn PaginationPrimitive(children: Children) -> impl IntoView {
    view! {
        <nav role="navigation" aria-label="pagination" data-slot="pagination">
            {children()}
        </nav>
    }
}

#[component]
pub fn PaginationContentPrimitive(children: Children) -> impl IntoView {
    view! {
        <ul data-slot="pagination-content">
            {children()}
        </ul>
    }
}

#[component]
pub fn PaginationItemPrimitive(children: Children) -> impl IntoView {
    view! {
        <li data-slot="pagination-item">
            {children()}
        </li>
    }
}

#[component]
pub fn PaginationLinkPrimitive(
    children: Children,
    #[prop(default = false)] is_active: bool,
) -> impl IntoView {
    view! {
        <a 
            aria-current=move || if is_active { Some("page") } else { None }
            data-slot="pagination-link"
            data-active=is_active
        >
            {children()}
        </a>
    }
}

#[component]
pub fn PaginationEllipsisPrimitive(children: Children) -> impl IntoView {
    view! {
        <span aria-hidden="true" data-slot="pagination-ellipsis">
            {children()}
        </span>
    }
}
