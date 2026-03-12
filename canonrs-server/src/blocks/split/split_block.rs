//! # Split Block

use leptos::prelude::*;

#[component]
pub fn Split(
    #[prop(optional)] aside: Option<ChildrenFn>,
    #[prop(optional)] main: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-block="split"
            data-block-version="1"
        >
            <div data-block-region="aside">{aside.map(|c| c())}</div>
            <div data-block-region="main">{main.map(|c| c())}</div>
        </div>
    }
}
