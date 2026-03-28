//! @canon-id: header
//! @canon-type: block
//! @canon-category: page
//! @canon-variant: page
//! @canon-container: true
//! @canon-regions: logo, nav, center, actions
//! @canon-label: Header
//! @canon-description: Page header with left center right regions
//! @canon-tags: header, top, nav, logo, title, page
//! @canon-slot-accepts: logo=Any,nav=Nav,center=Any,actions=Action
use leptos::prelude::*;

#[component]
pub fn Header(
    #[prop(optional)] logo: Option<ChildrenFn>,
    #[prop(optional)] nav: Option<ChildrenFn>,
    #[prop(optional)] center: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <header data-rs-block="" data-rs-component="Header" class=class>
            {logo.map(|l| view! { <div data-rs-region="logo">{l()}</div> })}
            {nav.map(|n| view! { <nav data-rs-region="nav">{n()}</nav> })}
            {center.map(|c| view! { <div data-rs-region="center">{c()}</div> })}
            {actions.map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
        </header>
    }
}
