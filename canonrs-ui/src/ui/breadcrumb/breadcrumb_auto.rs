//! BreadcrumbAuto - Automatic breadcrumb from NavigationContext

use leptos::prelude::*;
use crate::primitives::{BreadcrumbPrimitive, BreadcrumbItemPrimitive, BreadcrumbLinkPrimitive, BreadcrumbSeparatorPrimitive};
use super::navigation_provider::use_navigation_state;

#[component]
pub fn BreadcrumbAuto(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let nav_state = use_navigation_state();
    
    let breadcrumb_items = move || {
        let state = nav_state.get();
        
        if let Some(ref current_id) = state.current_heading_id {
            state.heading_hierarchy.get_breadcrumb(current_id)
        } else {
            Vec::new()
        }
    };
    
    view! {
        <BreadcrumbPrimitive class={class} id={id}>
            {move || {
                let items = breadcrumb_items();
                let len = items.len();
                
                items.into_iter().enumerate().map(|(idx, (id, text))| {
                    let is_last = idx == len - 1;
                    
                    view! {
                        <>
                            <BreadcrumbItemPrimitive>
                                <BreadcrumbLinkPrimitive 
                                    href={format!("#{}", id)}
                                    current={is_last}
                                >
                                    {text}
                                </BreadcrumbLinkPrimitive>
                            </BreadcrumbItemPrimitive>
                            {(!is_last).then(|| view! {
                                <BreadcrumbSeparatorPrimitive>"/"</BreadcrumbSeparatorPrimitive>
                            })}
                        </>
                    }
                }).collect_view()
            }}
        </BreadcrumbPrimitive>
    }
}
