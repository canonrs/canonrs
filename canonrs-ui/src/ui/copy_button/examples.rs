use leptos::prelude::*;
use super::copy_button_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 1rem;">
            <CopyButton text="Hello, World!".to_string() id="copy-example-1".to_string() />
            <CopyButton text="const x = 42;".to_string() id="copy-example-2".to_string() />
        </div>
    }
}
