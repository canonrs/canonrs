//! # EmptyState Block
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
            data-block="empty-state"
            data-block-version="1"
        >
            <div data-block-region="icon">
                {icon.map(|i| view! { <div class="canon-empty-state__icon">{i()}</div> })}
            </div>
            <div data-block-region="title">
                {title.map(|t| view! { <h3 class="canon-empty-state__title">{t}</h3> })}
            </div>
            <div data-block-region="description">
                {description.map(|d| view! { <p class="canon-empty-state__description">{d}</p> })}
            </div>
            <div data-block-region="action">
                {action.map(|a| view! { <div class="canon-empty-state__action">{a()}</div> })}
            </div>
        </div>
    }
}
