use leptos::prelude::*;

#[component]
pub fn MockWizardHeader() -> impl IntoView {
    view! { <div class="mock-region mock-region--header">"Header"</div> }
}

#[component]
pub fn MockWizardStepper() -> impl IntoView {
    view! { <div class="mock-region mock-region--stepper">"Stepper"</div> }
}

#[component]
pub fn MockWizardContent() -> impl IntoView {
    view! { <div class="mock-region mock-region--main">"Step Content"</div> }
}

#[component]
pub fn MockWizardFooter() -> impl IntoView {
    view! { <div class="mock-region mock-region--footer">"Footer Actions"</div> }
}
