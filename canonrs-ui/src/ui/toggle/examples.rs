use leptos::prelude::*;
use super::toggle_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Toggle pressed=false aria_label="Bold">
            "Bold"
        </Toggle>
    }
}
