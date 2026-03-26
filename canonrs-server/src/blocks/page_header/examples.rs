use leptos::prelude::*;
use super::page_header_block::PageHeader;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <PageHeader
            title=leptos::children::ToChildren::to_children(|| view!{ "Page Title" })
            subtitle=leptos::children::ToChildren::to_children(|| view!{ "Page description" })
            actions=leptos::children::ToChildren::to_children(|| view!{ <button>"Action"</button> })
        />
    }
}
