use leptos::prelude::*;
use super::carousel_ui::{Carousel, CarouselTrack, CarouselItem, CarouselPrev, CarouselNext, CarouselIndicators};
use canonrs_core::ToggleState;

#[island]
pub fn CarouselInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        let f = Closure::wrap(Box::new(move || {
            crate::interactions::carousel::init_all();
        }) as Box<dyn Fn()>);
        leptos::web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .ok();
        f.forget();
    }
    view! { <></> }
}

#[component]
pub fn CarouselIsland(
    children: Children,
    #[prop(optional)] initial_index: Option<usize>,
    #[prop(optional, into)] autoplay: Option<bool>,
    #[prop(optional)] interval: Option<u32>,
    #[prop(optional)] loop_state: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let ap = if autoplay.unwrap_or(false) { ToggleState::On } else { ToggleState::Off };
    let lp = if loop_state.unwrap_or(true) { ToggleState::On } else { ToggleState::Off };
    view! {
        <CarouselInit />
        <Carousel
            initial_index=initial_index.unwrap_or(0)
            autoplay=ap
            interval=interval.unwrap_or(5000)
            loop_state=lp
            class=class.unwrap_or_default()
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
