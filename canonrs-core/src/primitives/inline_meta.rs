//! @canon-level: strict
//! @canon-owner: primitives-team
//! InlineMeta Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn InlineMetaPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_im = crate::infra::uid::generate("im");
    view! {
        <span
            data-rs-inline-meta=""
            data-rs-uid=uid_im
            class=class
        >
            {children()}
        </span>
    }
}

#[component]
pub fn InlineMetaLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-inline-meta-label="" class=class>{children()}</span>
    }
}

#[component]
pub fn InlineMetaValuePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-inline-meta-value="" class=class>{children()}</span>
    }
}
