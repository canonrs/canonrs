use leptos::prelude::*;
use super::hero_island::{
    HeroTitleIsland, HeroSubtitleIsland, HeroDescriptionIsland,
    HeroMediaIsland, HeroActionsIsland,
};

#[component]
pub fn HeroShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <HeroTitleIsland>"Build faster with CanonRS"</HeroTitleIsland>
                <HeroSubtitleIsland>"A design system built for production."</HeroSubtitleIsland>
                <HeroDescriptionIsland>
                    "Composable, accessible, and fully typed components for Leptos."
                </HeroDescriptionIsland>
                <HeroActionsIsland>
                    <button>"Get Started"</button>
                    <button>"View Docs"</button>
                </HeroActionsIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Composable subcomponents — use only what you need."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With Media"</span>
                <HeroTitleIsland>"Visual storytelling"</HeroTitleIsland>
                <HeroMediaIsland>
                    <img src="/placeholder.png" alt="Hero media example" />
                </HeroMediaIsland>
            </div>
        </div>
    }
}
