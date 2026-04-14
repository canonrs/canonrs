//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavigationGroup Primitive - Semantic wrapper for navigation items

use leptos::prelude::*;

#[component]
pub fn NavigationGroupPrimitive(
    children: Children,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-navigation-group=""
            data-rs-uid=crate::infra::uid::generate("ng")
            data-rs-interaction="nav"
            data-rs-component="NavigationGroup"
            role="group"
            aria-labelledby=aria_labelledby
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn NavigationGroupLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-navigation-group-label=""
            class=class
        >
            {children()}
        </span>
    }
}
