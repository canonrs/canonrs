//! InlineNotice Boundary — Canon Rule passthrough
use leptos::prelude::*;
use super::inline_notice_ui::{InlineNotice as InlineNoticeUi, InlineNoticeIcon, InlineNoticeContent};
use canonrs_core::primitives::InlineNoticeVariant;

#[component]
pub fn InlineNotice(
    #[prop(optional, into)] content: Option<String>,
    #[prop(optional, into)] icon:    Option<String>,
    #[prop(default = InlineNoticeVariant::Default)] variant: InlineNoticeVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <InlineNoticeUi variant=variant class=class>
            {icon.map(|i| view! { <InlineNoticeIcon>{i}</InlineNoticeIcon> })}
            {content.map(|c| view! { <InlineNoticeContent>{c}</InlineNoticeContent> })}
        </InlineNoticeUi>
    }
}
