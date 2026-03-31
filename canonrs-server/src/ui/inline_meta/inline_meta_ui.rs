
use leptos::prelude::*;
use canonrs_core::primitives::{InlineMetaPrimitive, InlineMetaLabelPrimitive, InlineMetaValuePrimitive};

#[component]
pub fn InlineMeta(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InlineMetaPrimitive class=class>
            {children()}
        </InlineMetaPrimitive>
    }
}

#[component]
pub fn InlineMetaLabel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InlineMetaLabelPrimitive class=class>
            {children()}
        </InlineMetaLabelPrimitive>
    }
}

#[component]
pub fn InlineMetaValue(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InlineMetaValuePrimitive class=class>
            {children()}
        </InlineMetaValuePrimitive>
    }
}

#[component]
pub fn InlineMetaPreview() -> impl IntoView {
    view! {
        <InlineMeta>
            <InlineMetaLabel>"Rules"</InlineMetaLabel>
            <InlineMetaValue>"284"</InlineMetaValue>
        </InlineMeta>
    }
}
