// TableOfContents UI Component
// Navigation component for markdown headings

use leptos::prelude::*;
pub use crate::shared::TocItem;

#[component]
pub fn TableOfContents(items: Vec<TocItem>) -> impl IntoView {
    view! {
        <nav class="toc">
            <ul>
                {items.into_iter().map(|item| {
                    view! {
                        <li data-level={item.level.to_string()}>
                            <a href={format!("#{}", item.id)}>{item.text}</a>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </nav>
    }
}
