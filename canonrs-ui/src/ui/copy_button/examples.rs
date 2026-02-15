use leptos::prelude::*;
use super::copy_button_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 1rem; align-items: center;">
            <CopyButton
                text="Hello, World!"
                id="copy-example-1"
            />
            <CopyButton
                text="const x = 42;"
                id="copy-example-2"
            />
            <CopyButton
                text="fn main() { println!(\"Hello\"); }"
                id="copy-example-3"
            />
        </div>
    }
}
