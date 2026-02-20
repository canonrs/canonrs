//! # EmptyState Block
//! Empty state placeholder with optional action

use leptos::prelude::*;

#[component]
pub fn EmptyState(
    #[prop(optional)] icon: Option<ChildrenFn>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] action: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div 
            class=format!("canon-empty-state {}", class)
            attr:data-block="empty-state"
        >
            {icon.map(|i| view! {
                <div class="canon-empty-state__icon">{i()}</div>
            })}
            
            {title.map(|t| view! {
                <h3 class="canon-empty-state__title">{t}</h3>
            })}
            
            {description.map(|d| view! {
                <p class="canon-empty-state__description">{d}</p>
            })}
            
            {action.map(|a| view! {
                <div class="canon-empty-state__action">{a()}</div>
            })}
        </div>
    }
}
