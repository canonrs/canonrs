use leptos::prelude::*;
use super::skeleton_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <Skeleton class="h-4 w-full".to_string() />
            <Skeleton class="h-4 w-3/4".to_string() />
        </div>
    }
}
