//! @canon-id: card
//! @canon-type: block
//! @canon-category: layout
//! @canon-variant: structure
//! @canon-container: true
//! @canon-regions: header, content, footer
//! @canon-label: Card
//! @canon-description: Container with header/content/footer regions
//! @canon-tags: card, container, box, content
//! @canon-prop: padding | Number | 1rem | visual | padding
//! @canon-prop: gap | Number | 0.5rem | visual | gap
//! @canon-prop: border-radius | Number | 0.5rem | visual | border-radius
//! @canon-slot-accepts: header=Any,content=Any,footer=Action
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
            data-rs-block=""
            data-rs-component="Card"
            data-rs-variant=variant.as_str()
            style=style
            class=class
        >
            {header.map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
