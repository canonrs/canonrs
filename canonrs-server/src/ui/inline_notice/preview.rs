use leptos::prelude::*;
use super::inline_notice_boundary::InlineNotice;
use canonrs_core::primitives::InlineNoticeVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn InlineNoticeShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <InlineNotice variant=InlineNoticeVariant::Info icon="ℹ" content="This is an inline informational notice." />
            <p data-rs-showcase-preview-anchor="">
                "Role and aria-live automatically enforced by variant."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <InlineNotice variant=InlineNoticeVariant::Success icon="✓" content="Password updated successfully." />
                    <InlineNotice variant=InlineNoticeVariant::Warning icon="⚠" content="This field is required to continue." />
                    <InlineNotice variant=InlineNoticeVariant::Error   icon="✕" content="Invalid email address format." />
                </Stack>
            </Stack>
        </Stack>
    }
}
