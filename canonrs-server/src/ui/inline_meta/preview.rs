use leptos::prelude::*;
use super::inline_meta_ui::{InlineMeta, InlineMetaLabel, InlineMetaValue};

#[component]
pub fn InlineMetaShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InlineMeta>
                    <InlineMetaLabel>"Author"</InlineMetaLabel>
                    <InlineMetaValue>"Cristiano Bertulucci"</InlineMetaValue>
                </InlineMeta>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Metadata pairs structured and consistently rendered."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pairs"</span>
                <div data-rs-showcase-preview-row="">
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
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Context examples"</span>
                <div data-rs-showcase-preview-row="">
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
                </div>
            </div>
        </div>
    }
}
