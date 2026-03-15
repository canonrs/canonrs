use leptos::prelude::*;
use super::EmptyState;

pub fn basic_example() -> impl IntoView {
    view! {
        <EmptyState
            title="No results found".to_string()
            description="Try adjusting your search or filters.".to_string()
            action=leptos::children::ToChildren::to_children(|| view!{ <button>"Reset filters"</button> })
        />
    }
}
