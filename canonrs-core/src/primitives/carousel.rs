//! @canon-level: strict
//! @canon-owner: primitives-team
//! Carousel Primitive - Interactive slideshow

use leptos::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
#[allow(dead_code)]
static CAROUSEL_UID: AtomicU32 = AtomicU32::new(0);
use crate::meta::{ActivityState, DisabledState, VisibilityState};
use crate::infra::state_engine::{activity_attrs, disabled_attrs, visibility_attrs};

#[component]
pub fn CarouselPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-carousel=""
            data-rs-uid=CAROUSEL_UID.fetch_add(1, Ordering::SeqCst).to_string()
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
    #[prop(optional)] index: Option<usize>,
    #[prop(optional)] total: Option<usize>,
) -> impl IntoView {
    let aa  = activity_attrs(activity);
    let vis = visibility_attrs(visibility);
    let aria_label = match (index, total) {
        (Some(i), Some(t)) => Some(format!("Slide {} of {}", i + 1, t)),
        _ => None,
    };
    view! {
        <div
            data-rs-carousel-item=""
            data-rs-state=aa.data_rs_state
            role="group"
            aria-roledescription="slide"
            aria-label=aria_label
            aria-hidden=vis.aria_hidden
            hidden=vis.hidden
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
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-carousel-prev=""
            data-rs-disabled=d.data_rs_disabled
            disabled=d.disabled
            aria-disabled=d.aria_disabled
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
    let d = disabled_attrs(disabled);
    view! {
        <button
            type="button"
            data-rs-carousel-next=""
            data-rs-disabled=d.data_rs_disabled
            disabled=d.disabled
            aria-disabled=d.aria_disabled
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
    let aa = activity_attrs(state);
    view! {
        <button
            type="button"
            data-rs-carousel-dot=""
            data-rs-state=aa.data_rs_state
            aria-label=aria_label
            aria-current=if state == ActivityState::Active { Some("true") } else { None }
            class=class
        />
    }
}
