//! # Container Block
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ContainerSize { Sm, Md, #[default] Lg, Xl, Full }
impl ContainerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm", Self::Md => "md", Self::Lg => "lg",
            Self::Xl => "xl", Self::Full => "full",
        }
    }
}

#[component]
pub fn Container(
    #[prop(default = ContainerSize::Lg)] size: ContainerSize,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] style: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-block="container"
            data-block-version="1"
            data-block-size=size.as_str()
            class=class
            style=style
        >
            <div data-block-region="content">{children()}</div>
        </div>
    }
}
