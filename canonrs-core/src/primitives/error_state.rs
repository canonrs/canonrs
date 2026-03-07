//! @canon-level: strict
//! @canon-owner: primitives-team
//! ErrorState Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn ErrorStatePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-error-state=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ErrorStateIconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-error-state-icon=""
            aria-hidden="true"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ErrorStateTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <h3
            data-error-state-title=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </h3>
    }
}

#[component]
pub fn ErrorStateDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <p
            data-error-state-description=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn ErrorStateActionsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <div
            data-error-state-actions=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
