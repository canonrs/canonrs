use leptos::prelude::*;
use super::PageHeader;

pub fn basic_example() -> impl IntoView {
    view! {
        <PageHeader
            title="Page Title".to_string()
            subtitle="Page description".to_string()
            actions=leptos::children::ToChildren::to_children(|| view!{ <button>"Action"</button> })
        />
    }
}
