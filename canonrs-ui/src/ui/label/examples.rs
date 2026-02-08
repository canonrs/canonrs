use leptos::prelude::*;
use super::label_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Label for_id="input-example".to_string()>"Username"</Label>
    }
}
