use leptos::prelude::*;
use super::boundary::{Kbd, KbdGroup, KbdSeparator};
use super::kbd_ui::{KbdSize, KbdVariant};

#[component]
pub fn KbdShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <KbdGroup>
                    <Kbd>"Ctrl"</Kbd>
                    <KbdSeparator />
                    <Kbd>"K"</Kbd>
                </KbdGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Shortcut representation standardized via size and variant enums."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Size variants"</span>
                <div data-rs-showcase-preview-row="">
                    <Kbd size=KbdSize::Sm>"Sm"</Kbd>
                    <Kbd size=KbdSize::Md>"Md"</Kbd>
                    <Kbd size=KbdSize::Lg>"Lg"</Kbd>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variant"</span>
                <div data-rs-showcase-preview-row="">
                    <Kbd>"Default"</Kbd>
                    <Kbd variant=KbdVariant::Outline>"Outline"</Kbd>
                    <Kbd variant=KbdVariant::Ghost>"Ghost"</Kbd>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Common shortcuts"</span>
                <div data-rs-showcase-preview-row="">
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd>
                        <KbdSeparator />
                        <Kbd>"C"</Kbd>
                    </KbdGroup>
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd>
                        <KbdSeparator />
                        <Kbd>"Shift"</Kbd>
                        <KbdSeparator />
                        <Kbd>"P"</Kbd>
                    </KbdGroup>
                    <KbdGroup>
                        <Kbd>"⌘"</Kbd>
                        <KbdSeparator />
                        <Kbd>"Z"</Kbd>
                    </KbdGroup>
                </div>
            </div>
        </div>
    }
}
