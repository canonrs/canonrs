use leptos::prelude::*;

#[component]
pub fn MockSidebar() -> impl IntoView {
    view! { <div class="mock-region mock-region--sidebar">"Sidebar"</div> }
}

#[component]
pub fn MockAside() -> impl IntoView {
    view! { <div class="mock-region mock-region--aside">"Aside"</div> }
}

#[component]
pub fn MockMain() -> impl IntoView {
    view! { <div class="mock-region mock-region--main">"Main Content"</div> }
}
