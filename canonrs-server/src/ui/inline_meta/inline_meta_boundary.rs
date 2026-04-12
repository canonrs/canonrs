//! InlineMeta Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::inline_meta_ui::{
    InlineMeta as InlineMetaUi,
    InlineMetaLabel as InlineMetaLabelUi,
    InlineMetaValue as InlineMetaValueUi
};

#[component]
pub fn InlineMeta(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <InlineMetaUi class=class>{children()}</InlineMetaUi> }
}

#[component]
pub fn InlineMetaLabel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <InlineMetaLabelUi class=class>{children()}</InlineMetaLabelUi> }
}

#[component]
pub fn InlineMetaValue(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <InlineMetaValueUi class=class>{children()}</InlineMetaValueUi> }
}
