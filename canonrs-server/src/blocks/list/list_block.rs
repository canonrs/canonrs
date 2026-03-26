//! # List Block
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ListType { #[default] Unordered, Ordered }
impl ListType {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Unordered => "unordered", Self::Ordered => "ordered" }
    }
}

#[component]
pub fn List(
    #[prop(default = ListType::Unordered)] list_type: ListType,
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(optional)] items: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div
            data-block="list"
            data-block-version="1"
            data-block-type=list_type.as_str()
            class=class
        >
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {items.map(|i| view! { <div data-block-region="items">{i()}</div> })}
            {footer.map(|f| view! { <div data-block-region="footer">{f()}</div> })}
        </div>
    }
}
