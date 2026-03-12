//! # Wizard Block

use leptos::prelude::*;

#[component]
pub fn Wizard(
    #[prop(optional)] steps: Option<ChildrenFn>,
    #[prop(optional)] body: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-block="wizard"
            data-block-version="1"
        >
            <div data-block-region="steps">{steps.map(|c| c())}</div>
            <div data-block-region="body">{body.map(|c| c())}</div>
            <div data-block-region="actions">{actions.map(|c| c())}</div>
        </div>
    }
}
