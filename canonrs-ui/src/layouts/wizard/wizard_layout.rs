//! # WizardLayout — Regions: header, stepper, main, footer
use leptos::prelude::*;

#[component]
pub fn WizardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] stepper: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-wizard" data-layout="wizard" data-layout-version="1">
            <header class="layout-wizard-header" data-layout-region="header">{header.map(|h| h())}</header>
            <div class="layout-wizard-stepper" data-layout-region="stepper">{stepper.map(|s| s())}</div>
            <main class="layout-wizard-main" data-layout-region="main">{children()}</main>
            <footer class="layout-wizard-footer" data-layout-region="footer">{footer.map(|f| f())}</footer>
        </div>
    }
}
