use leptos::prelude::*;
use super::inline_meta_island::{InlineMetaIsland, InlineMetaLabelIsland, InlineMetaValueIsland};

#[component]
pub fn InlineMetaShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InlineMetaIsland>
                    <InlineMetaLabelIsland>"Author"</InlineMetaLabelIsland>
                    <InlineMetaValueIsland>"Cristiano Bertulucci"</InlineMetaValueIsland>
                </InlineMetaIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Metadata pairs structured and consistently rendered."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pairs"</span>
                <div data-rs-showcase-preview-row="">
                    <InlineMetaIsland>
                        <InlineMetaLabelIsland>"Status"</InlineMetaLabelIsland>
                        <InlineMetaValueIsland>"Active"</InlineMetaValueIsland>
                    </InlineMetaIsland>
                    <InlineMetaIsland>
                        <InlineMetaLabelIsland>"Version"</InlineMetaLabelIsland>
                        <InlineMetaValueIsland>"1.4.2"</InlineMetaValueIsland>
                    </InlineMetaIsland>
                    <InlineMetaIsland>
                        <InlineMetaLabelIsland>"License"</InlineMetaLabelIsland>
                        <InlineMetaValueIsland>"MIT"</InlineMetaValueIsland>
                    </InlineMetaIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Context examples"</span>
                <div data-rs-showcase-preview-row="">
                    <InlineMetaIsland>
                        <InlineMetaLabelIsland>"Rules"</InlineMetaLabelIsland>
                        <InlineMetaValueIsland>"284"</InlineMetaValueIsland>
                    </InlineMetaIsland>
                    <InlineMetaIsland>
                        <InlineMetaLabelIsland>"Components"</InlineMetaLabelIsland>
                        <InlineMetaValueIsland>"97"</InlineMetaValueIsland>
                    </InlineMetaIsland>
                    <InlineMetaIsland>
                        <InlineMetaLabelIsland>"Last updated"</InlineMetaLabelIsland>
                        <InlineMetaValueIsland>"2025-06-01"</InlineMetaValueIsland>
                    </InlineMetaIsland>
                </div>
            </div>
        </div>
    }
}
