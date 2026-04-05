use leptos::prelude::*;

#[island]
pub fn PaginationIsland(
    total_pages: u32,
    #[prop(optional)] current_page: Option<u32>,
    #[prop(optional, into)] base_href: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class     = class.unwrap_or_default();
    let base_href = base_href.unwrap_or_else(|| "#".to_string());
    let (page, set_page) = signal(current_page.unwrap_or(1));
    let initial_page = current_page.unwrap_or(1); // initial_state via initial_page
    let _ = set_page;

    let prev_disabled = move || page.get() <= 1;
    let next_disabled = move || page.get() >= total_pages;

    #[cfg(feature = "hydrate")]
    let on_prev = move |e: leptos::ev::MouseEvent| {
        e.prevent_default();
        if page.get_untracked() > 1 { set_page.update(|p| *p -= 1); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_prev = move |_: leptos::ev::MouseEvent| {};

    #[cfg(feature = "hydrate")]
    let on_next = move |e: leptos::ev::MouseEvent| {
        e.prevent_default();
        if page.get_untracked() < total_pages { set_page.update(|p| *p += 1); }
    };
    #[cfg(not(feature = "hydrate"))]
    let on_next = move |_: leptos::ev::MouseEvent| {};

    let pages_view = (1..=total_pages).map(|p| {
        let is_active  = move || page.get() == p;
        let state      = move || if is_active() { "active" } else { "inactive" };
        let initial    = if p == initial_page { "active" } else { "inactive" };
        let href       = format!("{}?page={}", base_href, p);
        #[cfg(feature = "hydrate")]
        let on_page = move |e: leptos::ev::MouseEvent| {
            e.prevent_default();
            set_page.set(p);
        };
        #[cfg(not(feature = "hydrate"))]
        let on_page = move |_: leptos::ev::MouseEvent| {};
        view! {
            <li data-rs-pagination-item="">
                <a
                    data-rs-pagination-link=""
                    href=href
                    data-rs-state=move || { let s = state(); if s.is_empty() { initial } else { s } }
                    aria-current=move || if is_active() { "page" } else { "false" }
                    on:click=on_page
                >
                    {p.to_string()}
                </a>
            </li>
        }
    }).collect::<Vec<_>>();

    view! {
        <nav
            data-rs-pagination=""
            data-rs-component="Pagination"
            aria-label="Pagination"
            class=class
        >
            <ul data-rs-pagination-content="">
                <li data-rs-pagination-item="">
                    <a
                        data-rs-pagination-previous=""
                        href="#"
                        data-rs-state=move || if prev_disabled() { "disabled" } else { "inactive" }
                        aria-disabled=move || prev_disabled().to_string()
                        on:click=on_prev
                    >
                        "←"
                    </a>
                </li>
                {pages_view}
                <li data-rs-pagination-item="">
                    <a
                        data-rs-pagination-next=""
                        href="#"
                        data-rs-state=move || if next_disabled() { "disabled" } else { "inactive" }
                        aria-disabled=move || next_disabled().to_string()
                        on:click=on_next
                    >
                        "→"
                    </a>
                </li>
            </ul>
        </nav>
    }
}
