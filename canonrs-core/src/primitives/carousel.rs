//! @canon-level: strict
//! @canon-owner: primitives-team
//! Carousel Primitive - Interactive slideshow

use leptos::prelude::*;

#[component]
pub fn CarouselPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-carousel=""
            role="region"
            aria-roledescription="carousel"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselTrackPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-carousel-track="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] active: bool,
    #[prop(optional)] index: Option<usize>,
    #[prop(optional)] total: Option<usize>,
) -> impl IntoView {
    let aria_label = match (index, total) {
        (Some(i), Some(t)) => Some(format!("Slide {} of {}", i + 1, t)),
        _ => None,
    };
    view! {
        <div
            data-rs-carousel-item=""
            data-rs-state=if active { "active" } else { "inactive" }
            role="group"
            aria-roledescription="slide"
            aria-label=aria_label
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselPrevPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-carousel-prev=""
            aria-label="Previous slide"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn CarouselNextPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-carousel-next=""
            aria-label="Next slide"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn CarouselIndicatorsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-carousel-indicators=""
            role="group"
            aria-label="Slide indicators"
            class=class
        >
            {children()}
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
            data-rs-state=if active { "active" } else { "inactive" }
            aria-label=aria_label
            aria-current=if active { Some("true") } else { None }
            class=class
        />
    }
}
