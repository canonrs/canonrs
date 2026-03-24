//! @canon-level: strict
//! @canon-owner: primitives-team
//! Carousel Primitive - Interactive slideshow

use leptos::prelude::*;

#[component]
pub fn CarouselPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-carousel=""
            role="region"
            aria-roledescription="carousel"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselTrackPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-carousel-track="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-carousel-item=""
            data-rs-active={if active { "true" } else { "false" }}
            role="group"
            aria-roledescription="slide"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselPrevPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-carousel-prev=""
            aria-label="Previous slide"
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn CarouselNextPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-carousel-next=""
            aria-label="Next slide"
            class=class
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn CarouselIndicatorsPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-carousel-indicators=""
            role="group"
            aria-label="Slide indicators"
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn CarouselDotPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] aria_label: String,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-carousel-dot=""
            data-rs-active={if active { "true" } else { "false" }}
            aria-label=aria_label
            aria-current={if active { "true" } else { "false" }}
            class=class
        />
    }
}
