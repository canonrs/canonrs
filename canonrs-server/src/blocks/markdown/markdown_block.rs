use leptos::prelude::*;

#[component]
pub fn MarkdownBlock(
    #[prop(optional, into)] content: Option<String>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class=class data-block="markdown" data-block-version="1">
            {content.map(|c| view! { <div inner_html=c /> })}
            {children.map(|ch| ch())}
        </div>
    }
}
