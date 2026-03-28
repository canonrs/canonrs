//! @canon-id: markdown
//! @canon-type: block
//! @canon-category: content
//! @canon-variant: feature
//! @canon-container: false
//! @canon-regions: header, content, footer
//! @canon-label: Markdown
//! @canon-description: Rendered markdown content block
//! @canon-tags: markdown, text, content, rich-text, document
//! @canon-slot-accepts: header=Any,content=Any,footer=Any
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
        <div data-rs-block="" data-rs-component="Markdown" class=class>
            {header.map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
