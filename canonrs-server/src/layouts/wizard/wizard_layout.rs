//! @canon-id: wizard-layout
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: header, stepper, content, footer
//! @canon-label: Wizard
//! @canon-icon: 📋
//! @canon-description: Multi-step form with header, stepper, content and footer
//! @canon-tags: wizard-layout, wizard, steps, onboarding, multi-step, form
//! @canon-slot-accepts: header=Any,stepper=Any,content=Form,footer=Action
//! @canon-slot-descriptions: header:Wizard title and progress,stepper:Step indicators,content:Step content,footer:Navigation actions
use leptos::prelude::*;

#[component]
pub fn WizardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] stepper: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-layout="" data-rs-component="Wizard" class=class>
            {header.map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {stepper.map(|s| view! { <div data-rs-region="stepper">{s()}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
