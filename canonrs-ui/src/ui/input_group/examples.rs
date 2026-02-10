use leptos::prelude::*;
use super::InputGroup;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <InputGroup>
            <span>"Input Group Example"</span>
        </InputGroup>
    }
}
