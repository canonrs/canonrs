//! # Breadcrumb Block
//! Navigation breadcrumb trail

use leptos::prelude::*;

#[derive(Clone)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

#[component]
pub fn Breadcrumb(
    #[prop(into)] items: Vec<BreadcrumbItem>,
    #[prop(default = "/".to_string(), into)] separator: String,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <nav 
            class=format!("canon-breadcrumb {}", class)
            attr:data-block="breadcrumb"
            attr:aria-label="Breadcrumb"
        >
            <ol class="canon-breadcrumb__list">
                {items.iter().enumerate().map(|(idx, item)| {
                    let is_last = idx == items.len() - 1;
                    let item_clone = item.clone();
                    
                    view! {
                        <li class="canon-breadcrumb__item">
                            {if is_last {
                                view! {
                                    <span 
                                        class="canon-breadcrumb__link canon-breadcrumb__link--current"
                                        attr:aria-current="page"
                                    >
                                        {item_clone.label}
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <a 
                                        href=item_clone.href.unwrap_or_else(|| "#".to_string())
                                        class="canon-breadcrumb__link"
                                    >
                                        {item_clone.label}
                                    </a>
                                    <span class="canon-breadcrumb__separator">{separator.clone()}</span>
                                }.into_any()
                            }}
                        </li>
                    }
                }).collect_view()}
            </ol>
        </nav>
    }
}
