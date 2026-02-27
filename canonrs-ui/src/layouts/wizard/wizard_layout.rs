//! # WizardLayout — Regions: header, stepper, main, footer
use leptos::prelude::*;

#[component]
pub fn WizardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] stepper: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = Signal::derive(|| String::new()))] header_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] stepper_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] main_zone_id: Signal<String>,
    #[prop(default = Signal::derive(|| String::new()))] footer_zone_id: Signal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="layout-wizard" data-layout="wizard" data-layout-version="1">
            <header class="layout-wizard-header"
                data-layout-region="header"
                data-region-hint="Drop branding or title"
                data-drop-zone=move || (!header_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!header_zone_id.get().is_empty()).then(|| header_zone_id.get())>
                {header.map(|h| h())}
            </header>
            <div class="layout-wizard-stepper"
                data-layout-region="stepper"
                data-region-hint="Drop step progress"
                data-drop-zone=move || (!stepper_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!stepper_zone_id.get().is_empty()).then(|| stepper_zone_id.get())>
                {stepper.map(|s| s())}
            </div>
            <main class="layout-wizard-main"
                data-layout-region="main"
                data-region-hint="Drop step content"
                data-drop-zone=move || (!main_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!main_zone_id.get().is_empty()).then(|| main_zone_id.get())>
                {children()}
            </main>
            <footer class="layout-wizard-footer"
                data-layout-region="footer"
                data-region-hint="Drop Back / Next actions"
                data-drop-zone=move || (!footer_zone_id.get().is_empty()).then_some("")
                data-zone-id=move || (!footer_zone_id.get().is_empty()).then(|| footer_zone_id.get())>
                {footer.map(|f| f())}
            </footer>
        </div>
    }
}
