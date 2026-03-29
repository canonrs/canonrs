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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InlineNoticePrimitive variant=variant class=class>
            {children()}
        </InlineNoticePrimitive>
    }
}

#[component]
pub fn InlineNoticeIcon(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InlineNoticeIconPrimitive class=class>
            {children()}
        </InlineNoticeIconPrimitive>
    }
}

#[component]
pub fn InlineNoticeContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InlineNoticeContentPrimitive class=class>
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
