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
                        <CarouselItemIsland active=true><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Slide 1"</div></CarouselItemIsland>
                        <CarouselItemIsland><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Slide 2"</div></CarouselItemIsland>
                        <CarouselItemIsland><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Slide 3"</div></CarouselItemIsland>
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
                            <CarouselItemIsland active=true><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Auto 1"</div></CarouselItemIsland>
                            <CarouselItemIsland><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Auto 2"</div></CarouselItemIsland>
                            <CarouselItemIsland><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Auto 3"</div></CarouselItemIsland>
                        </CarouselTrackIsland>
                        <CarouselPrevIsland>"←"</CarouselPrevIsland>
                        <CarouselNextIsland>"→"</CarouselNextIsland>
                    </CarouselIsland>
                </div>
            </div>
        </div>
    }
}
