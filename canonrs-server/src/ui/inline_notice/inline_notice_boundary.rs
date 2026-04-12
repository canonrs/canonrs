//! InlineNotice Island — Canon Rule passthrough
use leptos::prelude::*;
use canonrs_core::primitives::InlineNoticeVariant;

#[component]
pub fn InlineNotice(
    #[prop(optional, into)] content: Option<String>,
    #[prop(optional, into)] icon:    Option<String>,
    #[prop(default = InlineNoticeVariant::Default)] variant: InlineNoticeVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-inline-notice=""
            data-rs-component="InlineNotice"
            data-rs-variant=variant.as_str()
            role=variant.role()
            aria-live=variant.aria_live()
            class=class
        >
            {icon.map(|i| view! { <span data-rs-inline-notice-icon="">{i}</span> })}
            <span data-rs-inline-notice-content="">{content}</span>
        </div>
    }
}
