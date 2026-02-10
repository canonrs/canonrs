use leptos::prelude::*;
use super::Kbd;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display:flex;gap:0.5rem;align-items:center">
            <Kbd>"Ctrl"</Kbd>
            <span>"+"</span>
            <Kbd>"C"</Kbd>
        </div>
    }
}
