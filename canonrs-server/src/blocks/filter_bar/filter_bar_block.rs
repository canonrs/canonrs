//! # FilterBar Block
use leptos::prelude::*;

#[component]
pub fn FilterBar(
    #[prop(optional)] filters: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-block="filter-bar" data-block-version="1" class=class>
            {filters.map(|f| view! { <div data-block-region="filters">{f()}</div> })}
            {actions.map(|a| view! { <div data-block-region="actions">{a()}</div> })}
        </div>
    }
}
