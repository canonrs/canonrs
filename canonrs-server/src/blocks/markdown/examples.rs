use leptos::prelude::*;
use super::MarkdownBlock;

pub fn basic_example() -> impl IntoView {
    view! {
        <MarkdownBlock content="## Hello\n\nThis is **markdown** content.".to_string() />
    }
}
