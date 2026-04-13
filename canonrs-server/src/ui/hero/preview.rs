use leptos::prelude::*;
use canonrs_core::slot;
use crate::blocks::hero::hero_block::{Hero, HeroVariant};
use super::hero_boundary::{HeroTitle, HeroSubtitle, HeroDescription, HeroActions};
use crate::ui::button::button_boundary::Button;
use canonrs_core::primitives::ButtonVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn HeroShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Hero
                content=slot!(|| view! {
                    <HeroTitle>"Build faster with CanonRS"</HeroTitle>
                    <HeroSubtitle>"A design system built for production."</HeroSubtitle>
                    <HeroDescription>"Composable, accessible, and fully typed components for Leptos."</HeroDescription>
                }.into_any())
                actions=slot!(|| view! {
                    <HeroActions>
                        <Button variant=ButtonVariant::Primary>"Get Started"</Button>
                        <Button variant=ButtonVariant::Ghost>"View Docs"</Button>
                    </HeroActions>
                }.into_any())
            />
            <p data-rs-showcase-preview-anchor="">
                "Composable subcomponents — use only what you need."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Split variant"</span>
                <Hero
                    variant=HeroVariant::Split
                    content=slot!(|| view! {
                        <HeroTitle>"Visual storytelling"</HeroTitle>
                        <HeroSubtitle>"Media and content side by side."</HeroSubtitle>
                    }.into_any())
                    media=slot!(|| view! {
                        <img src="/placeholder.png" alt="Hero media example" />
                    }.into_any())
                />
            </Stack>
        </Stack>
    }
}
