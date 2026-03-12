//! # DetailPanel Block

use leptos::prelude::*;

#[component]
pub fn DetailPanel(
    #[prop(optional)] aside: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-block="detail-panel"
            data-block-version="1"
        >
            <div data-block-region="aside">{aside.map(|c| c())}</div>
            <div data-block-region="content">{content.map(|c| c())}</div>
        </div>
    }
}
