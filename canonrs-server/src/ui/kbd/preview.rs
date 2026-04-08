use leptos::prelude::*;
use super::kbd_island::{KbdIsland, KbdGroupIsland, KbdSeparatorIsland};
use super::kbd_ui::{KbdSize, KbdVariant};

#[component]
pub fn KbdShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <KbdGroupIsland>
                    <KbdIsland>"Ctrl"</KbdIsland>
                    <KbdSeparatorIsland />
                    <KbdIsland>"K"</KbdIsland>
                </KbdGroupIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Shortcut representation standardized via size and variant enums."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Size variants"</span>
                <div data-rs-showcase-preview-row="">
                    <KbdIsland size=KbdSize::Sm>"Sm"</KbdIsland>
                    <KbdIsland size=KbdSize::Md>"Md"</KbdIsland>
                    <KbdIsland size=KbdSize::Lg>"Lg"</KbdIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variant"</span>
                <div data-rs-showcase-preview-row="">
                    <KbdIsland>"Default"</KbdIsland>
                    <KbdIsland variant=KbdVariant::Outline>"Outline"</KbdIsland>
                    <KbdIsland variant=KbdVariant::Ghost>"Ghost"</KbdIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Common shortcuts"</span>
                <div data-rs-showcase-preview-row="">
                    <KbdGroupIsland>
                        <KbdIsland>"Ctrl"</KbdIsland>
                        <KbdSeparatorIsland />
                        <KbdIsland>"C"</KbdIsland>
                    </KbdGroupIsland>
                    <KbdGroupIsland>
                        <KbdIsland>"Ctrl"</KbdIsland>
                        <KbdSeparatorIsland />
                        <KbdIsland>"Shift"</KbdIsland>
                        <KbdSeparatorIsland />
                        <KbdIsland>"P"</KbdIsland>
                    </KbdGroupIsland>
                    <KbdGroupIsland>
                        <KbdIsland>"⌘"</KbdIsland>
                        <KbdSeparatorIsland />
                        <KbdIsland>"Z"</KbdIsland>
                    </KbdGroupIsland>
                </div>
            </div>
        </div>
    }
}
