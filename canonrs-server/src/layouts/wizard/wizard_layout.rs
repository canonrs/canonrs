use leptos::prelude::*;

#[component]
pub fn WizardLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] stepper: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let header  = StoredValue::new(header);
    let stepper = StoredValue::new(stepper);
    let content = StoredValue::new(content);
    let footer  = StoredValue::new(footer);
    view! {
        <div data-rs-layout="wizard" class=class>
            {move || header.get_value().map(|h| view! { <header data-rs-region="header">{h()}</header> })}
            {move || stepper.get_value().map(|s| view! { <div data-rs-region="stepper" role="group" aria-label="Wizard steps">{s()}</div> })}
            {move || content.get_value().map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {move || footer.get_value().map(|f| view! { <footer data-rs-region="footer">{f()}</footer> })}
        </div>
    }
}
