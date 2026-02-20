use leptos::prelude::*;

#[component]
pub fn MockMarketingHeader() -> impl IntoView {
    view! { <div class="mock-region mock-region--header">"Header"</div> }
}

#[component]
pub fn MockMarketingHero() -> impl IntoView {
    view! { <div class="mock-region mock-region--hero">"Hero"</div> }
}

#[component]
pub fn MockMarketingMain() -> impl IntoView {
    view! { <div class="mock-region mock-region--main">"Main Content"</div> }
}

#[component]
pub fn MockMarketingFooter() -> impl IntoView {
    view! { <div class="mock-region mock-region--footer">"Footer"</div> }
}
