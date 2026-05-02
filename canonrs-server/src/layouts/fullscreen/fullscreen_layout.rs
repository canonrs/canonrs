use leptos::prelude::*;

#[component]
pub fn FullscreenLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let header  = StoredValue::new(header);
    let content = StoredValue::new(content);
    view! {
        <div data-rs-layout="fullscreen" class=class>
            {move || header.get_value().map(|h| view! { <header data-rs-region="header">{h()}</header> })}
            {move || content.get_value().map(|c| view! { <main data-rs-region="content">{c()}</main> })}
        </div>
    }
}
