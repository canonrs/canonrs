//! @canon-id: wizard
//! @canon-type: block
//! @canon-category: form
//! @canon-variant: feature
//! @canon-container: true
//! @canon-regions: steps, body, actions
//! @canon-label: Wizard
//! @canon-description: Multi-step form wizard block
//! @canon-tags: wizard, steps, multi-step, onboarding, form
//! @canon-slot-accepts: steps=Any,body=Form,actions=Action
use leptos::prelude::*;

#[component]
pub fn Wizard(
    #[prop(optional)] steps: Option<ChildrenFn>,
    #[prop(optional)] body: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div data-rs-block="" data-rs-component="Wizard" class=class>
            {steps.map(|s| view! { <div data-rs-region="steps">{s()}</div> })}
            {body.map(|b| view! { <div data-rs-region="body">{b()}</div> })}
            {actions.map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
        </div>
    }
}
