use leptos::prelude::*;
use super::icon_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 1rem;">
            <Icon>"⭐"</Icon>
            <Icon>"❤️"</Icon>
            <Icon>"✓"</Icon>
        </div>
    }
}
