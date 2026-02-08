use leptos::prelude::*;
use super::kbd_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 1rem;">
            <Kbd>"Ctrl"</Kbd>
            <span>"+"</span>
            <Kbd>"C"</Kbd>
        </div>
    }
}
