//! # FilterBar Block

use leptos::prelude::*;

#[component]
pub fn FilterBar(
    #[prop(optional)] filters: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-block="filter-bar"
            data-block-version="1"
        >
            <div data-block-region="filters">{filters.map(|c| c())}</div>
            <div data-block-region="actions">{actions.map(|c| c())}</div>
        </div>
    }
}
