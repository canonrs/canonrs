//! Carousel Island — Canon Rule passthrough
use leptos::prelude::*;
use super::carousel_ui::{Carousel, CarouselTrack, CarouselItem, CarouselPrev, CarouselNext, CarouselIndicators};
use canonrs_core::ToggleState;

#[component]
pub fn CarouselIsland(
    children: Children,
    #[prop(default = 0)] initial_index: usize,
    #[prop(default = ToggleState::Off)] autoplay: ToggleState,
    #[prop(default = 5000)] interval: u32,
    #[prop(default = ToggleState::On)] loop_state: ToggleState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Carousel initial_index=initial_index autoplay=autoplay interval=interval loop_state=loop_state class=class>
            {children()}
        </Carousel>
    }
}

#[component]
pub fn CarouselTrackIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CarouselTrack class=class>{children()}</CarouselTrack> }
}

#[component]
pub fn CarouselItemIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! { <CarouselItem class=class active=active>{children()}</CarouselItem> }
}

#[component]
pub fn CarouselPrevIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CarouselPrev class=class>{children()}</CarouselPrev> }
}

#[component]
pub fn CarouselNextIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CarouselNext class=class>{children()}</CarouselNext> }
}

#[component]
pub fn CarouselIndicatorsIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CarouselIndicators class=class>{children()}</CarouselIndicators> }
}
