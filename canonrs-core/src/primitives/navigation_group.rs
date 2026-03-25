//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavigationGroup Primitive - Semantic wrapper for navigation items

use leptos::prelude::*;

#[component]
pub fn NavigationGroupPrimitive(
    children: Children,
    #[prop(optional)] aria_labelledby: Option<String>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-navigation-group=""
            role="group"
            aria-labelledby=aria_labelledby
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn NavigationGroupLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span
            data-rs-navigation-group-label=""
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </span>
    }
}
