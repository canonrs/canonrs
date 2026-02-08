use leptos::prelude::*;
use super::Textarea;

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <Textarea placeholder="Enter your message...".to_string() />
    }
}

#[component]
pub fn with_rows_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Textarea placeholder="3 rows".to_string() rows=3u32 />
            <Textarea placeholder="5 rows".to_string() rows=5u32 />
            <Textarea placeholder="10 rows".to_string() rows=10u32 />
        </div>
    }
}

#[component]
pub fn states_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Textarea placeholder="Normal textarea".to_string() />
            <Textarea placeholder="Disabled textarea".to_string() disabled=true />
            <Textarea placeholder="Readonly textarea".to_string() value="This is readonly".to_string() readonly=true />
            <Textarea placeholder="Required textarea".to_string() required=true />
        </div>
    }
}

#[component]
pub fn with_value_example() -> impl IntoView {
    view! {
        <Textarea
            value="This is pre-filled content in the textarea.".to_string()
            placeholder="Enter text...".to_string()
            rows=4u32
        />
    }
}
