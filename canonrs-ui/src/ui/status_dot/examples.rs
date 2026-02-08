use leptos::prelude::*;
use super::status_dot_ui::*;
use crate::shared::StatusVariant;

pub fn basic_example() -> impl IntoView {
    view! {
        <div class="flex items-center gap-2">
            <StatusDot variant=StatusVariant::Online />
            <span>"Online"</span>
        </div>
    }
}
