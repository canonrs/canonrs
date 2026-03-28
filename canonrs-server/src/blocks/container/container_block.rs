//! @canon-id: container
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: content
//! @canon-label: Container
//! @canon-description: Max-width centered container
//! @canon-tags: container, wrapper, max-width, center, layout
//! @canon-prop: max-width | Text | 1200px | visual | max-width
//! @canon-prop: padding | Number | | visual | padding
//! @canon-slot-accepts: content=Any
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
            data-rs-block=""
            data-rs-component="Container"
            data-rs-size=size.as_str()
            class=class
            style=style
        >
            <div data-rs-region="content">{children()}</div>
        </div>
    }
}
