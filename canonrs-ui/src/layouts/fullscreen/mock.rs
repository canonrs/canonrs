use leptos::prelude::*;

#[component]
pub fn MockFullscreenHeader() -> impl IntoView {
    view! { <div class="mock-region mock-region--header">"Header"</div> }
}

#[component]
pub fn MockFullscreenContent() -> impl IntoView {
    view! { <div class="mock-region mock-region--main">"Fullscreen Content"</div> }
}
