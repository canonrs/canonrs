//! @canon-level: strict
//! Carousel Primitives - Interactive slideshow

use leptos::prelude::*;

#[component]
pub fn CarouselPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-carousel=""
            role="region"
            aria-roledescription="carousel"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselTrackPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-carousel-track=""
            class={class}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-carousel-item=""
            role="group"
            aria-roledescription="slide"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselPrevPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            data-carousel-prev=""
            type="button"
            aria-label="Previous slide"
            class={class}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn CarouselNextPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            data-carousel-next=""
            type="button"
            aria-label="Next slide"
            class={class}
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn CarouselIndicatorsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-carousel-indicators=""
            role="group"
            aria-label="Slide indicators"
            class={class}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselDotPrimitive(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            data-carousel-dot=""
            type="button"
            class={class}
        />
    }
}
