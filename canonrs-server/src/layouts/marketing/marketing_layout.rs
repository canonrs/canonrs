//! @canon-id: marketing
//! @canon-type: layout
//! @canon-category: layout
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: header, hero, content, footer
//! @canon-label: Marketing
//! @canon-icon: 🌐
//! @canon-description: Public page with header, hero, main content and footer
//! @canon-tags: marketing, landing, hero, page, site
//! @canon-slot-accepts: header=Nav,hero=Any,content=Any,footer=Action
//! @canon-slot-descriptions: header:Site header with navigation,hero:Hero/banner section,content:Main content sections,footer:Site footer
use leptos::prelude::*;

#[component]
pub fn MarketingLayout(
    #[prop(optional)] header: Option<AnyView>,
    #[prop(optional)] hero: Option<AnyView>,
    #[prop(optional)] content: Option<AnyView>,
    #[prop(optional)] footer: Option<AnyView>,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-layout="" data-rs-component="Marketing" class=class>
            {header.map(|h| view! { <div data-rs-region="header">{h}</div> })}
            {hero.map(|h| view! { <div data-rs-region="hero">{h}</div> })}
            {content.map(|c| view! { <div data-rs-region="content">{c}</div> })}
            {footer.map(|f| view! { <div data-rs-region="footer">{f}</div> })}
        </div>
    }
}
