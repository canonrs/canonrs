use leptos::prelude::*;
use super::carousel_island::{
    CarouselIsland, CarouselTrackIsland, CarouselItemIsland,
    CarouselPrevIsland, CarouselNextIsland, CarouselIndicatorsIsland,
};

#[component]
pub fn CarouselShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CarouselIsland>
                    <CarouselTrackIsland>
                        <CarouselItemIsland>
                            <div style="padding:2rem;background:var(--theme-surface-raised);border-radius:var(--radius-md);text-align:center;">
                                "Slide 1 — Design Systems"
                            </div>
                        </CarouselItemIsland>
                        <CarouselItemIsland>
                            <div style="padding:2rem;background:var(--theme-surface-raised);border-radius:var(--radius-md);text-align:center;">
                                "Slide 2 — Primitives"
                            </div>
                        </CarouselItemIsland>
                        <CarouselItemIsland>
                            <div style="padding:2rem;background:var(--theme-surface-raised);border-radius:var(--radius-md);text-align:center;">
                                "Slide 3 — Behaviors"
                            </div>
                        </CarouselItemIsland>
                    </CarouselTrackIsland>
                    <CarouselPrevIsland>"←"</CarouselPrevIsland>
                    <CarouselNextIsland>"→"</CarouselNextIsland>
                    <CarouselIndicatorsIsland><span /></CarouselIndicatorsIsland>
                </CarouselIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Slide state and navigation semantics enforced via structured primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Autoplay"</span>
                <div data-rs-showcase-preview-row="">
                    <CarouselIsland autoplay=true interval=3000u32>
                        <CarouselTrackIsland>
                            <CarouselItemIsland>
                                <div style="padding:1.5rem;background:var(--theme-surface-raised);border-radius:var(--radius-md);text-align:center;">
                                    "Auto A"
                                </div>
                            </CarouselItemIsland>
                            <CarouselItemIsland>
                                <div style="padding:1.5rem;background:var(--theme-surface-raised);border-radius:var(--radius-md);text-align:center;">
                                    "Auto B"
                                </div>
                            </CarouselItemIsland>
                            <CarouselItemIsland>
                                <div style="padding:1.5rem;background:var(--theme-surface-raised);border-radius:var(--radius-md);text-align:center;">
                                    "Auto C"
                                </div>
                            </CarouselItemIsland>
                        </CarouselTrackIsland>
                        <CarouselIndicatorsIsland><span /></CarouselIndicatorsIsland>
                    </CarouselIsland>
                </div>
            </div>
        </div>
    }
}
