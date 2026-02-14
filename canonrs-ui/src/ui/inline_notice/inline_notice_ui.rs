use leptos::prelude::*;
use crate::primitives::{
    InlineNoticePrimitive,
    InlineNoticeIconPrimitive,
    InlineNoticeContentPrimitive,
};

pub use crate::primitives::InlineNoticeVariant;

#[component]
pub fn InlineNotice(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = InlineNoticeVariant::Default)] variant: InlineNoticeVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <InlineNoticePrimitive variant=variant class=class id=id>
            {children.map(|c| c())}
        </InlineNoticePrimitive>
    }
}

#[component]
pub fn InlineNoticeIcon(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <InlineNoticeIconPrimitive class=class id=id>
            {children.map(|c| c())}
        </InlineNoticeIconPrimitive>
    }
}

#[component]
pub fn InlineNoticeContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <InlineNoticeContentPrimitive class=class id=id>
            {children.map(|c| c())}
        </InlineNoticeContentPrimitive>
    }
}
