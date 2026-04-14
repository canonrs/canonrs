//! @canon-level: strict
//! @canon-owner: primitives-team
//! ErrorState Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn ErrorStatePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-error-state=""
            data-rs-uid=crate::infra::uid::generate("ers")
            role="status"
            aria-live="assertive"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ErrorStateIconPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-error-state-icon="" aria-hidden="true" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn ErrorStateTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <h3 data-rs-error-state-title="" class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn ErrorStateDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-error-state-description="" class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn ErrorStateActionsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-error-state-actions="" class=class>
            {children()}
        </div>
    }
}
