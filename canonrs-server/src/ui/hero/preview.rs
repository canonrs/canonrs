use leptos::prelude::*;
use super::hero_boundary::{
    HeroTitle, HeroSubtitle, HeroDescription,
    HeroMedia, HeroActions,
};

#[component]
pub fn HeroShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <HeroTitle>"Build faster with CanonRS"</HeroTitle>
                <HeroSubtitle>"A design system built for production."</HeroSubtitle>
                <HeroDescription>
                    "Composable, accessible, and fully typed components for Leptos."
                </HeroDescription>
                <HeroActions>
                    <button>"Get Started"</button>
                    <button>"View Docs"</button>
                </HeroActions>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Composable subcomponents — use only what you need."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With Media"</span>
                <HeroTitle>"Visual storytelling"</HeroTitle>
                <HeroMedia>
                    <img src="/placeholder.png" alt="Hero media example" />
                </HeroMedia>
            </div>
        </div>
    }
}
