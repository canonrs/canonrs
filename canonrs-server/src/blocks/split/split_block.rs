//! # Split Block
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SplitRatio { #[default] Half, OneThird, TwoThirds }
impl SplitRatio {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Half => "half", Self::OneThird => "one-third", Self::TwoThirds => "two-thirds" }
    }
}

#[component]
pub fn Split(
    #[prop(default = SplitRatio::Half)] ratio: SplitRatio,
    #[prop(optional)] aside: Option<ChildrenFn>,
    #[prop(optional)] main: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div data-block="split" data-block-version="1" data-block-ratio=ratio.as_str() class=class>
            {aside.map(|a| view! { <div data-block-region="aside">{a()}</div> })}
            {main.map(|m| view! { <div data-block-region="main">{m()}</div> })}
        </div>
    }
}
