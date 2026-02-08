use leptos::prelude::*;
use super::toggle_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Toggle pressed=false aria_label="Bold".to_string()>
            "Bold"
        </Toggle>
    }
}
