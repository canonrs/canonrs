//! # WizardLayout — Regions: header, stepper, main, footer (always rendered)
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
            <header class="layout-wizard-header"
                data-layout-region="header"
                data-region-hint="Drop branding or title"
                data-region-meta="Nav · max 1">
                {header.map(|h| h())}
            </header>
            <div class="layout-wizard-stepper"
                data-layout-region="stepper"
                data-region-hint="Drop step progress"
                data-region-meta="Nav · max 1">
                {stepper.map(|s| s())}
            </div>
            <main class="layout-wizard-main"
                data-layout-region="main"
                data-region-hint="Drop step content"
                data-region-meta="Content · ∞">
                {children()}
            </main>
            <footer class="layout-wizard-footer"
                data-layout-region="footer"
                data-region-hint="Drop Back / Next actions"
                data-region-meta="Nav · max 1">
                {footer.map(|f| f())}
            </footer>
        </div>
    }
}
