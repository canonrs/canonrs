use leptos::prelude::*;
use super::inline_notice_ui::{InlineNotice, InlineNoticeIcon, InlineNoticeContent};
use canonrs_core::primitives::InlineNoticeVariant;

#[component]
pub fn InlineNoticeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InlineNotice variant=InlineNoticeVariant::Info>
                    <InlineNoticeIcon>"ℹ"</InlineNoticeIcon>
                    <InlineNoticeContent>"This is an inline informational notice."</InlineNoticeContent>
                </InlineNotice>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Role and aria-live automatically enforced by variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <InlineNotice variant=InlineNoticeVariant::Success>
                        <InlineNoticeIcon>"✓"</InlineNoticeIcon>
                        <InlineNoticeContent>"Password updated successfully."</InlineNoticeContent>
                    </InlineNotice>
                    <InlineNotice variant=InlineNoticeVariant::Warning>
                        <InlineNoticeIcon>"⚠"</InlineNoticeIcon>
                        <InlineNoticeContent>"This field is required to continue."</InlineNoticeContent>
                    </InlineNotice>
                    <InlineNotice variant=InlineNoticeVariant::Error>
                        <InlineNoticeIcon>"✕"</InlineNoticeIcon>
                        <InlineNoticeContent>"Invalid email address format."</InlineNoticeContent>
                    </InlineNotice>
                </div>
            </div>
        </div>
    }
}
