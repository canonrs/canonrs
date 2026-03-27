//! @canon-id: marketing
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: header, hero, content, footer
//! @canon-label: Marketing
//! @canon-icon: 🌐
//! @canon-description: Public page with header, hero, main content and footer
//! @canon-slot-descriptions: header:Site header with navigation,hero:Hero/banner section,content:Main content sections,footer:Site footer
use leptos::prelude::*;

#[component]
pub fn MarketingLayout(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] hero: Option<ChildrenFn>,
    #[prop(optional)] content: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-layout="marketing" data-layout-version="1" class=class>
            {header.map(|h| view! { <div data-layout-region="header">{h()}</div> })}
            {hero.map(|h| view! { <div data-layout-region="hero">{h()}</div> })}
            {content.map(|c| view! { <div data-layout-region="content">{c()}</div> })}
            {footer.map(|f| view! { <div data-layout-region="footer">{f()}</div> })}
        </div>
    }
}
