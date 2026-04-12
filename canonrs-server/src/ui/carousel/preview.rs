use leptos::prelude::*;
use super::boundary::{Carousel, CarouselTrack, CarouselItem, CarouselPrev, CarouselNext};
use canonrs_core::ToggleState;

#[component]
pub fn CarouselShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Carousel>
                    <CarouselTrack>
                        <CarouselItem active=true><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Slide 1"</div></CarouselItem>
                        <CarouselItem><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Slide 2"</div></CarouselItem>
                        <CarouselItem><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Slide 3"</div></CarouselItem>
                    </CarouselTrack>
                    <CarouselPrev>"←"</CarouselPrev>
                    <CarouselNext>"→"</CarouselNext>
                </Carousel>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Sequential navigation with optional autoplay and loop."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Autoplay"</span>
                <div data-rs-showcase-preview-row="">
                    <Carousel autoplay=ToggleState::On interval=3000u32>
                        <CarouselTrack>
                            <CarouselItem active=true><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Auto 1"</div></CarouselItem>
                            <CarouselItem><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Auto 2"</div></CarouselItem>
                            <CarouselItem><div style="display:flex;align-items:center;justify-content:center;height:100%;font-size:var(--font-size-xl);">"Auto 3"</div></CarouselItem>
                        </CarouselTrack>
                        <CarouselPrev>"←"</CarouselPrev>
                        <CarouselNext>"→"</CarouselNext>
                    </Carousel>
                </div>
            </div>
        </div>
    }
}
