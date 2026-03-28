//! @canon-id: empty-state
//! @canon-type: block
//! @canon-category: content
//! @canon-variant: feature
//! @canon-container: false
//! @canon-regions: icon, title, description, action
//! @canon-label: Empty State
//! @canon-description: Empty state placeholder block
//! @canon-tags: empty-state, empty, placeholder, no-data, zero-state
//! @canon-slot-accepts: icon=Any,title=Any,description=Any,action=Action
use leptos::prelude::*;

#[component]
pub fn EmptyState(
    #[prop(optional)] icon: Option<ChildrenFn>,
    #[prop(optional)] title: Option<ChildrenFn>,
    #[prop(optional)] description: Option<ChildrenFn>,
    #[prop(optional)] action: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] _style: String,
) -> impl IntoView {
    view! {
        <div data-rs-block="" data-rs-component="EmptyState" class=class>
            {icon.map(|i| view! { <div data-rs-region="icon">{i()}</div> })}
            {title.map(|t| view! { <div data-rs-region="title">{t()}</div> })}
            {description.map(|d| view! { <div data-rs-region="description">{d()}</div> })}
            {action.map(|a| view! { <div data-rs-region="action">{a()}</div> })}
        </div>
    }
}
