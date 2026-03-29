//! @canon-id: inline-notice
//! @canon-label: Inline Notice
//! @canon-family: feedback
//! @canon-category: Feedback
//! @canon-intent: Show inline contextual feedback
//! @canon-description: Inline notice message
//! @canon-composable: true
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts: InlineNoticeIcon, InlineNoticeContent
//! @canon-tags: inline-notice, notice, warning, inline, message, info

use leptos::prelude::*;
use canonrs_core::primitives::{
    InlineNoticePrimitive, InlineNoticeIconPrimitive,
    InlineNoticeContentPrimitive,
};
pub use canonrs_core::primitives::InlineNoticeVariant;

#[component]
pub fn InlineNotice(
    children: Children,
    #[prop(default = InlineNoticeVariant::Default)] variant: InlineNoticeVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <InlineNoticePrimitive variant=variant class=class.unwrap_or_default()>
            {children()}
        </InlineNoticePrimitive>
    }
}

#[component]
pub fn InlineNoticeIcon(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <InlineNoticeIconPrimitive class={class.unwrap_or_default()}>
            {children()}
        </InlineNoticeIconPrimitive>
    }
}

#[component]
pub fn InlineNoticeContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <InlineNoticeContentPrimitive class={class.unwrap_or_default()}>
            {children()}
        </InlineNoticeContentPrimitive>
    }
}

#[component]
pub fn InlineNoticePreview() -> impl IntoView {
    view! {
        <InlineNotice variant=InlineNoticeVariant::Info>
            <InlineNoticeContent>"This is an inline notice message."</InlineNoticeContent>
        </InlineNotice>
    }
}
