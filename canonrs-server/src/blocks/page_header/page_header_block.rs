//! @canon-id: page-header
//! @canon-type: block
//! @canon-category: page
//! @canon-variant: page
//! @canon-container: false
//! @canon-regions: breadcrumb, title, subtitle, actions
//! @canon-label: Page Header
//! @canon-description: Page title and actions header block
//! @canon-tags: page-header, title, heading, actions, breadcrumb
//! @canon-slot-accepts: breadcrumb=Nav,title=Any,subtitle=Any,actions=Action
use leptos::prelude::*;

#[component]
pub fn PageHeader(
    #[prop(optional)] breadcrumb: Option<ChildrenFn>,
    #[prop(optional)] title: Option<ChildrenFn>,
    #[prop(optional)] subtitle: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-block=""
            data-rs-component="PageHeader"
            class=class
        >
            {breadcrumb.map(|b| view! { <div data-rs-region="breadcrumb">{b()}</div> })}
            {title.map(|t| view! { <div data-rs-region="title">{t()}</div> })}
            {subtitle.map(|s| view! { <div data-rs-region="subtitle">{s()}</div> })}
            {actions.map(|a| view! { <div data-rs-region="actions">{a()}</div> })}
        </div>
    }
}
