use leptos::prelude::*;
use canonrs_core::infra::uid::generate;

#[component]
pub fn MarketingLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] hero: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = generate("ly");
    let header  = StoredValue::new(header);
    let hero    = StoredValue::new(hero);
    let content = StoredValue::new(content);
    let footer  = StoredValue::new(footer);
    view! {
        <div data-rs-layout="marketing" data-rs-uid=uid class=class>
            {move || header.get_value().map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {move || hero.get_value().map(|h| view! { <div data-rs-region="hero">{h()}</div> })}
            {move || content.get_value().map(|c| view! { <div data-rs-region="content">{c()}</div> })}
            {move || footer.get_value().map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
