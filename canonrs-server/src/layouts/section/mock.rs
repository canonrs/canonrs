use leptos::prelude::*;

#[component]
pub fn MockSectionHeader() -> impl IntoView {
    view! { <div class="mock-region mock-region--header">"Section Header"</div> }
}

#[component]
pub fn MockSectionContent() -> impl IntoView {
    view! { <div class="mock-region mock-region--main">"Section Content"</div> }
}

#[component]
pub fn MockSectionFooter() -> impl IntoView {
    view! { <div class="mock-region mock-region--footer">"Section Footer"</div> }
}
