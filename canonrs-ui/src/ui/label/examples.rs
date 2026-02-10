use leptos::prelude::*;
use super::Label;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Label for_id="example-input">"Username"</Label>
    }
}
