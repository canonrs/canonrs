use leptos::prelude::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BreadcrumbIslandItem {
    pub label:    String,
    pub href:     Option<String>,
    pub is_page:  bool,
}

#[island]
pub fn BreadcrumbIsland(
    items: Vec<BreadcrumbIslandItem>,
    #[prop(optional, into)] separator: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class     = class.unwrap_or_default();
    let separator = separator.unwrap_or_else(|| "/".to_string());
    let len       = items.len();

    let items_view = items.into_iter().enumerate().map(|(idx, item)| {
        let is_last = idx == len - 1;
        let sep     = separator.clone();

        let link_view = if item.is_page || is_last {
            view! {
                <span data-rs-breadcrumb-page="" aria-current="page">
                    {item.label}
                </span>
            }.into_any()
        } else {
            let href = item.href.unwrap_or_default();
            view! {
                <a data-rs-breadcrumb-link="" href=href data-rs-state="inactive">
                    {item.label}
                </a>
            }.into_any()
        };

        view! {
            <li data-rs-breadcrumb-item="">
                {link_view}
            </li>
            {if !is_last {
                view! {
                    <li data-rs-breadcrumb-separator="" aria-hidden="true">{sep}</li>
                }.into_any()
            } else {
                view! { <></> }.into_any()
            }}
        }
    }).collect::<Vec<_>>();

    view! {
        <nav
            data-rs-breadcrumb=""
            data-rs-component="Breadcrumb"
            aria-label="Breadcrumb"
            class=class
        >
            <ol data-rs-breadcrumb-list="">
                {items_view}
            </ol>
        </nav>
    }
}
