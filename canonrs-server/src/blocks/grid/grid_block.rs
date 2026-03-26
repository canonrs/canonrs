//! # Grid Block
use leptos::prelude::*;

#[component]
pub fn Grid(
    #[prop(default = 3u8)] columns: u8,
    #[prop(default = String::new(), into)] class: String,
    #[prop(optional)] items: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="grid"
            data-block-version="1"
            data-block-columns=columns.to_string()
            class=class
        >
            {items.map(|i| view! { <div data-block-region="items">{i()}</div> })}
        </div>
    }
}
