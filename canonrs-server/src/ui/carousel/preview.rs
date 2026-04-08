use leptos::prelude::*;
use super::carousel_island::{CarouselIsland, CarouselTrackIsland, CarouselItemIsland, CarouselPrevIsland, CarouselNextIsland};
use canonrs_core::ToggleState;

#[component]
pub fn CarouselShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CarouselIsland>
                    <CarouselTrackIsland>
                        <CarouselItemIsland active=true>"Slide 1"</CarouselItemIsland>
                        <CarouselItemIsland>"Slide 2"</CarouselItemIsland>
                        <CarouselItemIsland>"Slide 3"</CarouselItemIsland>
                    </CarouselTrackIsland>
                    <CarouselPrevIsland>"←"</CarouselPrevIsland>
                    <CarouselNextIsland>"→"</CarouselNextIsland>
                </CarouselIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Sequential navigation with optional autoplay and loop."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Autoplay"</span>
                <div data-rs-showcase-preview-row="">
                    <CarouselIsland autoplay=ToggleState::On interval=3000u32>
                        <CarouselTrackIsland>
                            <CarouselItemIsland active=true>"Auto 1"</CarouselItemIsland>
                            <CarouselItemIsland>"Auto 2"</CarouselItemIsland>
                            <CarouselItemIsland>"Auto 3"</CarouselItemIsland>
                        </CarouselTrackIsland>
                        <CarouselPrevIsland>"←"</CarouselPrevIsland>
                        <CarouselNextIsland>"→"</CarouselNextIsland>
                    </CarouselIsland>
                </div>
            </div>
        </div>
    }
}
