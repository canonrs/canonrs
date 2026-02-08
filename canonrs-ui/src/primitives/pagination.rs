//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pagination Primitive - Navigation for paginated content

use leptos::prelude::*;

#[component]
pub fn PaginationPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <nav data-pagination="" role="navigation" attr:aria-label="pagination" class={class} id={id}>
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn PaginationContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ul data-pagination-content="" class={class} id={id}>
            {children.map(|c| c())}
        </ul>
    }
}

#[component]
pub fn PaginationItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <li data-pagination-item="" class={class} id={id}>
            {children.map(|c| c())}
        </li>
    }
}

#[component]
pub fn PaginationLinkPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = false)] is_active: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let final_href = if href.is_empty() { "#".to_string() } else { href };
    
    view! {
        <a data-pagination-link="" attr:data-active={if is_active { "true" } else { "" }} href={final_href} attr:aria-current={if is_active { "page" } else { "" }} class={class} id={id}>
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationPreviousPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let final_href = if href.is_empty() { "#".to_string() } else { href };
    
    view! {
        <a data-pagination-previous="" attr:aria-label="Go to previous page" href={final_href} class={class} id={id}>
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationNextPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let final_href = if href.is_empty() { "#".to_string() } else { href };
    
    view! {
        <a data-pagination-next="" attr:aria-label="Go to next page" href={final_href} class={class} id={id}>
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationEllipsisPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span data-pagination-ellipsis="" attr:aria-hidden="true" class={class} id={id}>
            "…"
            <span class="sr-only">"More pages"</span>
        </span>
    }
}
