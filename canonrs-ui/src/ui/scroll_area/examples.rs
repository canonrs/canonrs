use leptos::prelude::*;
use super::scroll_area_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <ScrollArea class="h-[200px]".to_string()>
            <div style="height: 600px; padding: 1rem;">
                "Scrollable content. Lorem ipsum dolor sit amet."
            </div>
        </ScrollArea>
    }
}
