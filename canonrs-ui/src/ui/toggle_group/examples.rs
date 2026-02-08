use leptos::prelude::*;
use super::toggle_group_ui::*;
use crate::ui::toggle::Toggle;

pub fn basic_example() -> impl IntoView {
    view! {
        <ToggleGroup multiple=false>
            <Toggle pressed=true aria_label="Bold".to_string()>"B"</Toggle>
            <Toggle pressed=false aria_label="Italic".to_string()>"I"</Toggle>
            <Toggle pressed=false aria_label="Underline".to_string()>"U"</Toggle>
        </ToggleGroup>
    }
}
