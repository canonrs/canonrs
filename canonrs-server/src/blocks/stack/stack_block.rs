//! # Stack Block
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum StackDirection { #[default] Vertical, Horizontal }
impl StackDirection {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Vertical => "vertical", Self::Horizontal => "horizontal" }
    }
}

#[component]
pub fn Stack(
    #[prop(default = StackDirection::Vertical)] direction: StackDirection,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    #[prop(optional)] items: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="stack"
            data-block-version="1"
            data-block-direction=direction.as_str()
            class=class
            style=style
        >
            {items.map(|i| view! { <div data-block-region="items">{i()}</div> })}
        </div>
    }
}
