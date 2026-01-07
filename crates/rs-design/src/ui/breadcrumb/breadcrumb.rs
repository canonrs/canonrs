use leptos::prelude::*;
use super::{BreadcrumbNav, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};

/// Breadcrumb item data
#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItemData {
    pub label: String,
    pub href: Option<String>,
    pub is_current: bool,
}

/// Breadcrumb - Styled navigation breadcrumb
/// 
/// **Type:** Pure Component (Type 1)
/// **Tokens:** 100% Canonical
#[component]
pub fn Breadcrumb(
    /// Breadcrumb items (path from root to current)
    #[prop(into)]
    items: Signal<Vec<BreadcrumbItemData>>,
    
    /// Optional click handler (for SPA navigation)
    #[prop(optional)]
    on_navigate: Option<Callback<String>>,
    
    /// Optional custom separator text (default: "/")
    #[prop(optional, into)]
    separator: Option<String>,
    
    /// Optional CSS class
    #[prop(optional, into)]
    class: String,
) -> impl IntoView {
    let sep = separator.unwrap_or_else(|| "/".to_string());
    
    view! {
        <BreadcrumbNav class=format!("flex {}", class)>
            <BreadcrumbList class="flex flex-wrap items-center gap-1.5 text-sm break-words sm:gap-2.5 text-muted-foreground">
                {move || {
                    let items_list = items.get();
                    let total = items_list.len();
                    let separator_text = sep.clone();
                    
                    items_list.into_iter().enumerate().map(|(idx, item)| {
                        let is_last = idx == total - 1;
                        let sep_clone = separator_text.clone();
                        
                        view! {
                            <BreadcrumbItem class="inline-flex items-center gap-1.5">
                                {if item.is_current || is_last {
                                    view! {
                                        <BreadcrumbPage class="text-foreground font-normal">
                                            {item.label.clone()}
                                        </BreadcrumbPage>
                                    }.into_any()
                                } else if let Some(href) = item.href.clone() {
                                    if let Some(nav) = on_navigate {
                                        let h = href.clone();
                                        view! {
                                            <BreadcrumbLink
                                                href=href
                                                class="hover:text-foreground transition-colors"
                                                on_click=Callback::new(move |_| nav.run(h.clone()))
                                            >
                                                {item.label.clone()}
                                            </BreadcrumbLink>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <BreadcrumbLink
                                                href=href
                                                class="hover:text-foreground transition-colors"
                                            >
                                                {item.label.clone()}
                                            </BreadcrumbLink>
                                        }.into_any()
                                    }
                                } else {
                                    view! {
                                        <span class="text-muted-foreground">{item.label.clone()}</span>
                                    }.into_any()
                                }}
                            </BreadcrumbItem>
                            
                            {if !is_last {
                                view! {
                                    <BreadcrumbSeparator class="[&>svg]:size-3.5">
                                        {Some(move || view! { <span>{sep_clone.clone()}</span> })}
                                    </BreadcrumbSeparator>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        }
                    }).collect_view()
                }}
            </BreadcrumbList>
        </BreadcrumbNav>
    }
}
