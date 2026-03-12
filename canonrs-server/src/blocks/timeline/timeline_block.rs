//! # Timeline Block

use leptos::prelude::*;

#[component]
pub fn Timeline(
    #[prop(optional)] items: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-block="timeline"
            data-block-version="1"
        >
            <div data-block-region="items">{items.map(|c| c())}</div>
        </div>
    }
}
