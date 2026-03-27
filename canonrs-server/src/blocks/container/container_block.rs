//! @canon-id: container
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: content
//! @canon-prop: max-width | Text | 1200px | visual | max-width
//! @canon-prop: padding | Number | | visual | padding
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
