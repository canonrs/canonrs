//! @canon-id: hero
//! @canon-type: block
//! @canon-category: page
//! @canon-variant: centered
//! @canon-container: true
//! @canon-regions: header, media, content, actions, footer
//! @canon-label: Hero
//! @canon-description: Page hero block with media, content and actions regions
//! @canon-tags: hero, landing, intro, cta, media, page
//! @canon-slot-accepts: header=Any,media=Any,content=Any,actions=Action,footer=Any
use leptos::prelude::*;

#[component]
pub fn Hero(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] media: Option<ChildrenFn>,
    content: ChildrenFn,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-block=""
            data-rs-component="Hero"
            data-rs-behavior="structural"
            class=class
        >
            {header.map(|h| view! { <div data-rs-region="header">{h()}</div> })}
            {media.map(|m| view! { <div data-rs-region="media">{m()}</div> })}
            <div data-rs-region="content">{content()}</div>
            {actions.map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f()}</div> })}
        </div>
    }
}
