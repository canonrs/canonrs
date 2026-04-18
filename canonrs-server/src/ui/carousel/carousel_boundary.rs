//! Carousel Island — Canon Rule passthrough
use leptos::prelude::*;
use super::carousel_ui::{
    Carousel as CarouselUi,
    CarouselTrack as CarouselTrackUi,
    CarouselItem as CarouselItemUi,
    CarouselPrev as CarouselPrevUi,
    CarouselNext as CarouselNextUi,
    CarouselIndicators as CarouselIndicatorsUi,
    CarouselDot as CarouselDotUi
};
use canonrs_core::ToggleState;

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
        <CarouselUi initial_index=initial_index autoplay=autoplay interval=interval loop_state=loop_state class=class>
            {children()}
        </CarouselUi>
    }
}

#[component]
pub fn CarouselTrack(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CarouselTrackUi class=class>{children()}</CarouselTrackUi> }
}

#[component]
pub fn CarouselItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! { <CarouselItemUi class=class active=active>{children()}</CarouselItemUi> }
}

#[component]
pub fn CarouselPrev(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CarouselPrevUi class=class>{children()}</CarouselPrevUi> }
}

#[component]
pub fn CarouselNext(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CarouselNextUi class=class>{children()}</CarouselNextUi> }
}

#[component]
pub fn CarouselIndicators(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <CarouselIndicatorsUi class=class>{children()}</CarouselIndicatorsUi> }
}

#[component]
pub fn CarouselDot(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! { <CarouselDotUi class=class aria_label=aria_label active=active /> }
}
