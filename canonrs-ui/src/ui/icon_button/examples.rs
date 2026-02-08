use leptos::prelude::*;
use super::icon_button_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 1rem; align-items: center;">
            <IconButton size="sm" aria_label="Edit".to_string()>"✏️"</IconButton>
            <IconButton size="md" aria_label="Delete".to_string()>"🗑️"</IconButton>
            <IconButton size="lg" aria_label="Share".to_string()>"🔗"</IconButton>
        </div>
    }
}
