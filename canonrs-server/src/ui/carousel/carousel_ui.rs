
use leptos::prelude::*;
use canonrs_core::ToggleState;
use canonrs_core::primitives::{
    CarouselPrimitive, CarouselTrackPrimitive, CarouselItemPrimitive,
    CarouselPrevPrimitive, CarouselNextPrimitive,
    CarouselIndicatorsPrimitive,
};

#[component]
pub fn Carousel(
    children: Children,
    #[prop(default = 0)] initial_index: usize,
    #[prop(default = ToggleState::Off)] autoplay: ToggleState,
    #[prop(default = 5000)] interval: u32,
    #[prop(default = ToggleState::On)] loop_state: ToggleState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselPrimitive
            class={class}
        >
            <div
                data-rs-carousel-wrapper=""
                data-rs-autoplay={if autoplay == ToggleState::On { Some("") } else { None }}
                data-rs-loop={if loop_state == ToggleState::On { Some("") } else { None }}
                data-rs-initial-index={initial_index.to_string()}
                data-rs-interval={interval.to_string()}
            >
                {children()}
            </div>
        </CarouselPrimitive>
    }
}

#[component]
pub fn CarouselTrack(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselTrackPrimitive class={class}>
            {children()}
        </CarouselTrackPrimitive>
    }
}

#[component]
pub fn CarouselItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselItemPrimitive class={class}>
            {children()}
        </CarouselItemPrimitive>
    }
}

#[component]
pub fn CarouselPrev(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselPrevPrimitive class={class}>
            {children()}
        </CarouselPrevPrimitive>
    }
}

#[component]
pub fn CarouselNext(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselNextPrimitive class={class}>
            {children()}
        </CarouselNextPrimitive>
    }
}

#[component]
pub fn CarouselIndicators(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselIndicatorsPrimitive class={class}>
            {children()}
        </CarouselIndicatorsPrimitive>
    }
}

#[component]
pub fn CarouselPreview() -> impl IntoView {
    view! {
        <Carousel>
            <CarouselTrack>
                <CarouselItem>"Slide 1"</CarouselItem>
            </CarouselTrack>
        </Carousel>
    }
}
