//! @canon-id: markdown
//! @canon-type: block
//! @canon-category: content
//! @canon-variant: feature
//! @canon-container: false
//! @canon-regions: header, content, footer
use leptos::prelude::*;

#[component]
pub fn MarkdownBlock(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
    #[prop(optional)] content: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div data-block="markdown" data-block-version="1" class=class>
            {header.map(|h| view! { <div data-block-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-block-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-block-region="footer">{f()}</div> })}
        </div>
    }
}
