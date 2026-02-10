use leptos::prelude::*;
use super::Button;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Button>"Click me"</Button>
    }
}
