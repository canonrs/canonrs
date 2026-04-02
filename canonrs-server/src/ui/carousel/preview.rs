use leptos::prelude::*;
use super::carousel_ui::{Carousel, CarouselTrack, CarouselItem, CarouselPrev, CarouselNext, CarouselIndicators};
use canonrs_core::ToggleState;

#[component]
pub fn CarouselShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Carousel>
                    <CarouselTrack>
                        <CarouselItem>
                            <div style="padding:2rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);text-align:center;">
                                "Slide 1 — Design Systems"
                            </div>
                        </CarouselItem>
                        <CarouselItem>
                            <div style="padding:2rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);text-align:center;">
                                "Slide 2 — Primitives"
                            </div>
                        </CarouselItem>
                        <CarouselItem>
                            <div style="padding:2rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);text-align:center;">
                                "Slide 3 — Behaviors"
                            </div>
                        </CarouselItem>
                    </CarouselTrack>
                    <CarouselPrev>"←"</CarouselPrev>
                    <CarouselNext>"→"</CarouselNext>
                    <CarouselIndicators>
                        <span data-rs-carousel-dot="" />
                        <span data-rs-carousel-dot="" />
                        <span data-rs-carousel-dot="" />
                    </CarouselIndicators>
                </Carousel>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Slide state and navigation semantics enforced via structured primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Autoplay"</span>
                <div data-rs-showcase-preview-row="">
                    <Carousel autoplay=ToggleState::On interval=3000u32>
                        <CarouselTrack>
                            <CarouselItem>
                                <div style="padding:1.5rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);text-align:center;">
                                    "Auto A"
                                </div>
                            </CarouselItem>
                            <CarouselItem>
                                <div style="padding:1.5rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);text-align:center;">
                                    "Auto B"
                                </div>
                            </CarouselItem>
                            <CarouselItem>
                                <div style="padding:1.5rem;background:var(--rs-color-surface-raised);border-radius:var(--rs-radius-md);text-align:center;">
                                    "Auto C"
                                </div>
                            </CarouselItem>
                        </CarouselTrack>
                        <CarouselIndicators>
                            <span data-rs-carousel-dot="" />
                            <span data-rs-carousel-dot="" />
                            <span data-rs-carousel-dot="" />
                        </CarouselIndicators>
                    </Carousel>
                </div>
            </div>
        </div>
    }
}
