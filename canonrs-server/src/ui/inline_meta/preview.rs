use leptos::prelude::*;
use super::inline_meta_boundary::{InlineMeta, InlineMetaLabel, InlineMetaValue};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn InlineMetaShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <InlineMeta>
                <InlineMetaLabel>"Author"</InlineMetaLabel>
                <InlineMetaValue>"Cristiano Bertulucci"</InlineMetaValue>
            </InlineMeta>
            <p data-rs-showcase-preview-anchor="">
                "Metadata pairs structured and consistently rendered."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Pairs"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <InlineMeta>
                        <InlineMetaLabel>"Status"</InlineMetaLabel>
                        <InlineMetaValue>"Active"</InlineMetaValue>
                    </InlineMeta>
                    <InlineMeta>
                        <InlineMetaLabel>"Version"</InlineMetaLabel>
                        <InlineMetaValue>"1.4.2"</InlineMetaValue>
                    </InlineMeta>
                    <InlineMeta>
                        <InlineMetaLabel>"License"</InlineMetaLabel>
                        <InlineMetaValue>"MIT"</InlineMetaValue>
                    </InlineMeta>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Context examples"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                    <InlineMeta>
                        <InlineMetaLabel>"Rules"</InlineMetaLabel>
                        <InlineMetaValue>"284"</InlineMetaValue>
                    </InlineMeta>
                    <InlineMeta>
                        <InlineMetaLabel>"Components"</InlineMetaLabel>
                        <InlineMetaValue>"97"</InlineMetaValue>
                    </InlineMeta>
                    <InlineMeta>
                        <InlineMetaLabel>"Last updated"</InlineMetaLabel>
                        <InlineMetaValue>"2025-06-01"</InlineMetaValue>
                    </InlineMeta>
                </Stack>
            </Stack>
        </Stack>
    }
}
