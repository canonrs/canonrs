//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavigationGroup Primitive - Semantic wrapper for navigation items

use leptos::prelude::*;

#[component]
pub fn NavigationGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] labelledby: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-navigation-group=""
            role="group"
            attr:aria-labelledby={(!labelledby.is_empty()).then(|| labelledby)}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn NavigationGroupLabelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-navigation-group-label=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}
