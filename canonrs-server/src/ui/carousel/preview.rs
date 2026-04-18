use leptos::prelude::*;
use super::carousel_boundary::{
    Carousel, CarouselTrack, CarouselItem,
    CarouselPrev, CarouselNext, CarouselIndicators, CarouselDot
};
use canonrs_core::ToggleState;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CarouselShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Carousel>
                <CarouselTrack>
                    <CarouselItem active=true>
                        <img src="/assets/canonrs-image1.webp" alt="Slide 1" style="width:100%;height:auto;display:block;" />
                    </CarouselItem>
                    <CarouselItem>
                        <img src="/assets/canonrs-image2.webp" alt="Slide 2" style="width:100%;height:auto;display:block;" />
                    </CarouselItem>
                    <CarouselItem>
                        <img src="/assets/canonrs-image3.webp" alt="Slide 3" style="width:100%;height:auto;display:block;" />
                    </CarouselItem>
                </CarouselTrack>
                <CarouselPrev>"←"</CarouselPrev>
                <CarouselNext>"→"</CarouselNext>
                <CarouselIndicators>
                    <CarouselDot active=true aria_label="Slide 1" />
                    <CarouselDot aria_label="Slide 2" />
                    <CarouselDot aria_label="Slide 3" />
                </CarouselIndicators>
            </Carousel>
            <p data-rs-showcase-preview-anchor="">
                "Sequential navigation with optional autoplay and loop."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Autoplay"</span>
                <Carousel autoplay=ToggleState::On interval=3000u32>
                    <CarouselTrack>
                        <CarouselItem active=true>
                            <img src="/assets/canonrs-image1.webp" alt="Slide 1" style="width:100%;height:auto;display:block;" />
                        </CarouselItem>
                        <CarouselItem>
                            <img src="/assets/canonrs-image2.webp" alt="Slide 2" style="width:100%;height:auto;display:block;" />
                        </CarouselItem>
                        <CarouselItem>
                            <img src="/assets/canonrs-image3.webp" alt="Slide 3" style="width:100%;height:auto;display:block;" />
                        </CarouselItem>
                    </CarouselTrack>
                    <CarouselPrev>"←"</CarouselPrev>
                    <CarouselNext>"→"</CarouselNext>
                    <CarouselIndicators>
                        <CarouselDot active=true aria_label="Slide 1" />
                        <CarouselDot aria_label="Slide 2" />
                        <CarouselDot aria_label="Slide 3" />
                    </CarouselIndicators>
                </Carousel>
            </Stack>
        </Stack>
    }
}
