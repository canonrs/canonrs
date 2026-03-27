//! @canon-id: carousel
//! @canon-label: Carousel
//! @canon-family: data_display
//! @canon-category: Display
//! @canon-intent: Cycle through items horizontally
//! @canon-description: Image carousel slider
//! @canon-composable: true
//! @canon-capabilities: KeyboardArrows
//! @canon-required-parts: CarouselTrack, CarouselItem
//! @canon-optional-parts: CarouselPrev, CarouselNext, CarouselIndicators
//! @canon-tags: carousel, slider, gallery, images, slideshow

use leptos::prelude::*;
use canonrs_core::primitives::{
    CarouselPrimitive, CarouselTrackPrimitive, CarouselItemPrimitive,
    CarouselPrevPrimitive, CarouselNextPrimitive,
    CarouselIndicatorsPrimitive,
};

#[component]
pub fn Carousel(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 0)] initial_index: usize,
    #[prop(default = false)] autoplay: bool,
    #[prop(default = 5000)] interval: u32,
    #[prop(default = true)] r#loop: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselPrimitive
            class={class}
        >
            <div
                data-rs-carousel-wrapper=""
                data-rs-autoplay={autoplay.then_some("")}
                data-rs-loop={r#loop.then_some("")}
                data-rs-initial-index={initial_index.to_string()}
                data-rs-interval={interval.to_string()}
            >
                {children.map(|c| c())}
            </div>
        </CarouselPrimitive>
    }
}

#[component]
pub fn CarouselTrack(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselTrackPrimitive class={class}>
            {children.map(|c| c())}
        </CarouselTrackPrimitive>
    }
}

#[component]
pub fn CarouselItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselItemPrimitive class={class}>
            {children.map(|c| c())}
        </CarouselItemPrimitive>
    }
}

#[component]
pub fn CarouselPrev(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselPrevPrimitive class={class}>
            {children.map(|c| c()).unwrap_or_else(|| view! { "‹" }.into_any())}
        </CarouselPrevPrimitive>
    }
}

#[component]
pub fn CarouselNext(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselNextPrimitive class={class}>
            {children.map(|c| c()).unwrap_or_else(|| view! { "›" }.into_any())}
        </CarouselNextPrimitive>
    }
}

#[component]
pub fn CarouselIndicators(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CarouselIndicatorsPrimitive class={class}>
            {children.map(|c| c())}
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
