use leptos::prelude::*;
use super::error_state_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <ErrorState
            title="Something went wrong".to_string()
            message="We encountered an error. Please try again.".to_string()
        />
    }
}
