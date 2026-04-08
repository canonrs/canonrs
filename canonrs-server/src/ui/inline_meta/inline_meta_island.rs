//! InlineMeta Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::inline_meta_ui::{InlineMeta, InlineMetaLabel, InlineMetaValue};

#[component]
pub fn InlineMetaIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <InlineMeta class=class>{children()}</InlineMeta> }
}

#[component]
pub fn InlineMetaLabelIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <InlineMetaLabel class=class>{children()}</InlineMetaLabel> }
}

#[component]
pub fn InlineMetaValueIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <InlineMetaValue class=class>{children()}</InlineMetaValue> }
}
