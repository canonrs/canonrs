use leptos::prelude::*;

#[component]
pub fn MarketingLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] hero: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let header  = StoredValue::new(header);
    let hero    = StoredValue::new(hero);
    let content = StoredValue::new(content);
    let footer  = StoredValue::new(footer);
    view! {
        <div data-rs-layout="marketing" class=class>
            {move || header.get_value().map(|h| view! { <header data-rs-region="header">{h()}</header> })}
            {move || hero.get_value().map(|h| view! { <div data-rs-region="hero" role="banner">{h()}</div> })}
            {move || content.get_value().map(|c| view! { <main data-rs-region="content">{c()}</main> })}
            {move || footer.get_value().map(|f| view! { <footer data-rs-region="footer">{f()}</footer> })}
        </div>
    }
}
