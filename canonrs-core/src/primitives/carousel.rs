//! @canon-level: strict
//! @canon-owner: primitives-team
//! Carousel Primitive - Interactive slideshow

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState, VisibilityState};

#[component]
pub fn CarouselPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("cr");
    view! {
        <div
            data-rs-carousel=""
            data-rs-uid=uid
            data-rs-interaction="gesture"
            role="region"
            aria-roledescription="carousel"
            aria-label=aria_label
            class=class
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
        <div
            data-rs-carousel-track=""
            role="group"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ActivityState::Inactive)] activity: ActivityState,
    #[prop(default = VisibilityState::Closed)] visibility: VisibilityState,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-carousel-item=""
            data-rs-activity=activity.as_str()
            role="group"
            aria-roledescription="slide"
            aria-label=aria_label
            aria-hidden=visibility.aria_hidden()
            hidden=visibility.hidden()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CarouselPrevPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-carousel-prev=""
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            disabled=disabled.disabled()
            aria-disabled=disabled.aria_disabled()
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
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-carousel-next=""
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            disabled=disabled.disabled()
            aria-disabled=disabled.aria_disabled()
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
    #[prop(default = ActivityState::Inactive)] state: ActivityState,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-carousel-dot=""
            data-rs-activity=state.as_str()
            aria-label=aria_label
            class=class
        />
    }
}
