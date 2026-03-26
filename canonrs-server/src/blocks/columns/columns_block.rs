//! # Columns Block
use leptos::prelude::*;

#[component]
pub fn Columns(
    #[prop(default = 2)] count: u8,
    #[prop(default = String::new(), into)] class: String,
    #[prop(optional)] columns: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="columns"
            data-block-version="1"
            data-block-columns=count.to_string()
            class=class
        >
            {columns.map(|c| view! { <div data-block-region="columns">{c()}</div> })}
        </div>
    }
}
