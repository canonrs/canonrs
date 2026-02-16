//! BreadcrumbAuto - Automatic breadcrumb from NavigationContext

use leptos::prelude::*;
use crate::primitives::{BreadcrumbPrimitive, BreadcrumbItemPrimitive, BreadcrumbLinkPrimitive};
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
            <For
                each=breadcrumb_items
                key=|(id, _)| id.clone()
                children=move |(id, text)| {
                    let id_clone = id.clone();
                    let is_last = move || {
                        let items = breadcrumb_items();
                        items.last().map(|(last_id, _)| last_id == &id_clone).unwrap_or(false)
                    };
                    
                    view! {
                        <BreadcrumbItemPrimitive>
                            <BreadcrumbLinkPrimitive 
                                href={format!("#{}", id)}
                                current={is_last()}
                            >
                                {text}
                            </BreadcrumbLinkPrimitive>
                        </BreadcrumbItemPrimitive>
                    }
                }
            />
        </BreadcrumbPrimitive>
    }
}
