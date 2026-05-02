//! @canon-level: strict
//! @canon-owner: primitives-team
//! Hero Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn HeroPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_hr = crate::infra::uid::generate("hr");
    view! {
        <div
            data-rs-block=""
            data-rs-uid=uid_hr
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn HeroMediaPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-hero-media="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn HeroActionsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-hero-actions="" class=class>
            {children()}
        </div>
    }
}
