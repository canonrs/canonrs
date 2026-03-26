use leptos::prelude::*;
use super::markdown_block::MarkdownBlock;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <MarkdownBlock
            content=leptos::children::ToChildren::to_children(|| view!{
                <p>"Hello "<strong>"markdown"</strong>" content."</p>
            })
        />
    }
}
