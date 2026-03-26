use leptos::prelude::*;
use super::empty_state_block::EmptyState;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <EmptyState
            title=leptos::children::ToChildren::to_children(|| view!{ "No results found" })
            description=leptos::children::ToChildren::to_children(|| view!{ "Try adjusting your search or filters." })
            action=leptos::children::ToChildren::to_children(|| view!{ <button>"Reset"</button> })
        />
    }
}
