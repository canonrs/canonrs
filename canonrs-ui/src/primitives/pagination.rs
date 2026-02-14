//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pagination Primitive - Navigation for paginated content

use leptos::prelude::*;

#[component]
pub fn PaginationPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <nav
            data-pagination=""
            
            attr:aria-label="Page navigation"
            class={if class.is_empty() { None } else { Some(class) }}
            id=id
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn PaginationContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ul data-pagination-content="" class={if class.is_empty() { None } else { Some(class) }} id=id>
            {children.map(|c| c())}
        </ul>
    }
}

#[component]
pub fn PaginationItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <li data-pagination-item="" class={if class.is_empty() { None } else { Some(class) }} id=id>
            {children.map(|c| c())}
        </li>
    }
}

#[component]
pub fn PaginationLinkPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] is_active: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let data_active = if is_active { Some("true") } else { None };
    let aria_current = if is_active { Some("page") } else { None };

    view! {
        
        <a
            data-pagination-link=""
            attr:data-active=data_active
            attr:aria-current=aria_current
            href=href
            class={if class.is_empty() { None } else { Some(class) }}
            id=id
        >
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationPreviousPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let data_disabled = if disabled { Some("true") } else { None };
    let aria_disabled = if disabled { Some("true") } else { None };
    let resolved_href = if disabled { "#".to_string() } else { href };

    view! {
        <a
        
            data-pagination-previous=""
            attr:data-disabled=data_disabled
            attr:aria-disabled=aria_disabled
            attr:aria-label="Go to previous page"
            href=resolved_href
            class={if class.is_empty() { None } else { Some(class) }}
            id=id
        >
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationNextPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let data_disabled = if disabled { Some("true") } else { None };
    let aria_disabled = if disabled { Some("true") } else { None };
    let resolved_href = if disabled { "#".to_string() } else { href };

    view! {
        <a
        
            data-pagination-next=""
            attr:data-disabled=data_disabled
            attr:aria-disabled=aria_disabled
            attr:aria-label="Go to next page"
            href=resolved_href
            class={if class.is_empty() { None } else { Some(class) }}
            id=id
        >
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn PaginationEllipsisPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span data-pagination-ellipsis="" aria-hidden="true" class={if class.is_empty() { None } else { Some(class) }} id=id>
            "…"
            <span class="sr-only">"More pages"</span>
        </span>
    }
}
