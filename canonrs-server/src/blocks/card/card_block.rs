//! # Card Block
use leptos::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum CardVariant { #[default] Default, Interactive, Outlined, Elevated }
impl CardVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default", Self::Interactive => "interactive",
            Self::Outlined => "outlined", Self::Elevated => "elevated",
        }
    }
}

#[component]
pub fn Card(
    #[prop(default = CardVariant::Default)] variant: CardVariant,
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    #[prop(optional)] content: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="card"
            data-block-version="1"
            style=style
            data-block-variant=variant.as_str()
            class=class
        >
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-block-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-block-region="footer">{f()}</div> })}
        </div>
    }
}
