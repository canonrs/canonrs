//! @canon-level: strict
//! @canon-owner: primitives-team
//! InlineNotice Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn InlineNoticePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-inline-notice=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn InlineNoticeIconPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-inline-notice-icon=""
            attr:aria-hidden="true"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn InlineNoticeContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-inline-notice-content=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}
