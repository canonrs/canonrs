use leptos::prelude::*;
use super::{Breadcrumb, BreadcrumbItemData};
use crate::primitives::{BreadcrumbPrimitive, BreadcrumbItemPrimitive, BreadcrumbLinkPrimitive, BreadcrumbSeparatorPrimitive};

#[component]
pub fn BasicExample() -> impl IntoView {
    let items = vec![
        BreadcrumbItemData { label: "Home".to_string(), href: "/".to_string(), is_current: false },
        BreadcrumbItemData { label: "Products".to_string(), href: "/products".to_string(), is_current: false },
        BreadcrumbItemData { label: "Category".to_string(), href: "/products/category".to_string(), is_current: false },
        BreadcrumbItemData { label: "Item".to_string(), href: "/products/category/item".to_string(), is_current: true },
    ];
    
    let hidden_items = vec![
        BreadcrumbItemData { label: "Documentation".to_string(), href: "/docs".to_string(), is_current: false },
        BreadcrumbItemData { label: "Components".to_string(), href: "/docs/components".to_string(), is_current: false },
        BreadcrumbItemData { label: "Forms".to_string(), href: "/docs/components/forms".to_string(), is_current: false },
    ];
    
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            <div>
                <h4>"Basic Breadcrumb"</h4>
                <Breadcrumb items=items />
            </div>
            
            <div>
                <h4>"Collapsible Breadcrumb"</h4>
                <BreadcrumbPrimitive class=String::new() id=String::new()>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbLinkPrimitive href="/".to_string() current=false class=String::new()>"Home"</BreadcrumbLinkPrimitive>
                    </BreadcrumbItemPrimitive>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbSeparatorPrimitive class=String::new()>"/"</BreadcrumbSeparatorPrimitive>
                    </BreadcrumbItemPrimitive>
                    
                    <details data-breadcrumb-collapse="" id="breadcrumb-collapse-1">
                        <summary data-breadcrumb-ellipsis-trigger="">"..."</summary>
                        <div data-breadcrumb-collapse-content="">
                            {hidden_items.into_iter().map(|item| {
                                view! {
                                    <BreadcrumbLinkPrimitive href={item.href} current=false class=String::new()>
                                        {item.label}
                                    </BreadcrumbLinkPrimitive>
                                }
                            }).collect_view()}
                        </div>
                    </details>
                    
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbSeparatorPrimitive class=String::new()>"/"</BreadcrumbSeparatorPrimitive>
                    </BreadcrumbItemPrimitive>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbLinkPrimitive href="/products".to_string() current=false class=String::new()>"Products"</BreadcrumbLinkPrimitive>
                    </BreadcrumbItemPrimitive>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbSeparatorPrimitive class=String::new()>"/"</BreadcrumbSeparatorPrimitive>
                    </BreadcrumbItemPrimitive>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <span data-breadcrumb-page="" aria-current="page">"Current Item"</span>
                    </BreadcrumbItemPrimitive>
                </BreadcrumbPrimitive>
            </div>
        </div>
    }
}
