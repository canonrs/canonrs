use leptos::prelude::*;
use super::carousel_boundary::{Carousel, CarouselTrack, CarouselItem, CarouselPrev, CarouselNext};
use canonrs_core::ToggleState;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CarouselShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Carousel>
                <CarouselTrack>
                    <CarouselItem active=true><div data-rs-carousel-demo="">"Slide 1"</div></CarouselItem>
                    <CarouselItem><div data-rs-carousel-demo="">"Slide 2"</div></CarouselItem>
                    <CarouselItem><div data-rs-carousel-demo="">"Slide 3"</div></CarouselItem>
                </CarouselTrack>
                <CarouselPrev>"←"</CarouselPrev>
                <CarouselNext>"→"</CarouselNext>
            </Carousel>
            <p data-rs-showcase-preview-anchor="">
                "Sequential navigation with optional autoplay and loop."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Autoplay"</span>
                <Carousel autoplay=ToggleState::On interval=3000u32>
                    <CarouselTrack>
                        <CarouselItem active=true><div data-rs-carousel-demo="">"Auto 1"</div></CarouselItem>
                        <CarouselItem><div data-rs-carousel-demo="">"Auto 2"</div></CarouselItem>
                        <CarouselItem><div data-rs-carousel-demo="">"Auto 3"</div></CarouselItem>
                    </CarouselTrack>
                    <CarouselPrev>"←"</CarouselPrev>
                    <CarouselNext>"→"</CarouselNext>
                </Carousel>
            </Stack>
        </Stack>
    }
}
