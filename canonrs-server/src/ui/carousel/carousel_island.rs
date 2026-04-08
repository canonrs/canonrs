use leptos::prelude::*;
use super::carousel_ui::{Carousel, CarouselTrack, CarouselItem, CarouselPrev, CarouselNext, CarouselIndicators};
use canonrs_core::ToggleState;

#[island]
pub fn CarouselInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::carousel::init_all();
        });
    }
    view! { <></> }
}

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
        <CarouselInit />
        <Carousel
            initial_index=initial_index
            autoplay=autoplay
            interval=interval
            loop_state=loop_state
            class=class
        >
            {children()}
        </Carousel>
    }
}

#[component]
pub fn CarouselTrackIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <CarouselTrack class=class.unwrap_or_default()>{children()}</CarouselTrack> }
}

#[component]
pub fn CarouselItemIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] active: Option<bool>,
) -> impl IntoView {
    view! { <CarouselItem class=class.unwrap_or_default() active=active.unwrap_or(false)>{children()}</CarouselItem> }
}

#[component]
pub fn CarouselPrevIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <CarouselPrev class=class.unwrap_or_default()>{children()}</CarouselPrev> }
}

#[component]
pub fn CarouselNextIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <CarouselNext class=class.unwrap_or_default()>{children()}</CarouselNext> }
}

#[component]
pub fn CarouselIndicatorsIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <CarouselIndicators class=class.unwrap_or_default()>{children()}</CarouselIndicators> }
}
