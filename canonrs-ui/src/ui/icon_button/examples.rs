use leptos::prelude::*;
use super::IconButton;
use crate::ui::icon::IconSize;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 1rem;">
            <IconButton size=IconSize::Sm aria_label="Edit".to_string()>"✏️"</IconButton>
            <IconButton size=IconSize::Md aria_label="Delete".to_string()>"🗑️"</IconButton>
            <IconButton size=IconSize::Lg aria_label="Share".to_string()>"🔗"</IconButton>
        </div>
    }
}
