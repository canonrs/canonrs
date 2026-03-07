use leptos::prelude::*;

#[component]
pub fn MockSplitLeft() -> impl IntoView {
    view! { <div class="mock-region mock-region--sidebar">"Left Panel"</div> }
}

#[component]
pub fn MockSplitRight() -> impl IntoView {
    view! { <div class="mock-region mock-region--main">"Right Panel"</div> }
}
