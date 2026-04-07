use leptos::prelude::*;
use super::inline_meta_ui::{InlineMeta, InlineMetaLabel, InlineMetaValue};

#[component]
pub fn InlineMetaIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <InlineMeta class=class.unwrap_or_default()>{children()}</InlineMeta> }
}

#[component]
pub fn InlineMetaLabelIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <InlineMetaLabel class=class.unwrap_or_default()>{children()}</InlineMetaLabel> }
}

#[component]
pub fn InlineMetaValueIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <InlineMetaValue class=class.unwrap_or_default()>{children()}</InlineMetaValue> }
}
