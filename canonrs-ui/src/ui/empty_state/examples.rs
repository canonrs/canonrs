use leptos::prelude::*;
use super::empty_state_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <EmptyState>
            <EmptyStateTitle>"No items found"</EmptyStateTitle>
            <EmptyStateDescription>"Try adjusting your search or filters."</EmptyStateDescription>
        </EmptyState>
    }
}
