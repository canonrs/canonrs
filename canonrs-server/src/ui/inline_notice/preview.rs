use leptos::prelude::*;
use super::inline_notice_island::{InlineNoticeIsland, InlineNoticeIslandVariant};

#[component]
pub fn InlineNoticeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InlineNoticeIsland variant=InlineNoticeIslandVariant::Info icon="ℹ" content="This is an inline informational notice." />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Role and aria-live automatically enforced by variant."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <InlineNoticeIsland variant=InlineNoticeIslandVariant::Success icon="✓" content="Password updated successfully." />
                    <InlineNoticeIsland variant=InlineNoticeIslandVariant::Warning icon="⚠" content="This field is required to continue." />
                    <InlineNoticeIsland variant=InlineNoticeIslandVariant::Error   icon="✕" content="Invalid email address format." />
                </div>
            </div>
        </div>
    }
}
