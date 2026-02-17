use leptos::prelude::*;
use crate::primitives::{
    CarouselPrimitive, CarouselTrackPrimitive, CarouselItemPrimitive,
    CarouselPrevPrimitive, CarouselNextPrimitive, 
    CarouselIndicatorsPrimitive, CarouselDotPrimitive
};

#[component]
pub fn Carousel(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 0)] initial_index: usize,
    #[prop(default = false)] autoplay: bool,
    #[prop(default = 5000)] interval: u32,
    #[prop(default = true)] r#loop: bool,
    #[prop(default = true)] show_controls: bool,
    #[prop(default = true)] show_indicators: bool,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <CarouselPrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            <div
                data-carousel-wrapper=""
                data-autoplay={autoplay.then_some("")}
                data-loop={r#loop.then_some("")}
                data-initial-index={initial_index.to_string()}
                data-interval={interval.to_string()}
            >
                {if show_controls {
                    view! { <CarouselPrev /> }.into_any()
                } else {
                    view! {}.into_any()
                }}

                <CarouselTrack>
                    {children.map(|c| c())}
                </CarouselTrack>

                {if show_controls {
                    view! { <CarouselNext /> }.into_any()
                } else {
                    view! {}.into_any()
                }}

                {if show_indicators {
                    view! { <CarouselIndicators /> }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
        </CarouselPrimitive>
    }
}

#[component]
pub fn CarouselTrack(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <CarouselTrackPrimitive>
            {children.map(|c| c())}
        </CarouselTrackPrimitive>
    }
}

#[component]
pub fn CarouselItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <CarouselItemPrimitive
            class={class}
            id={id.unwrap_or_default()}
        >
            {children.map(|c| c())}
        </CarouselItemPrimitive>
    }
}

#[component]
pub fn CarouselPrev() -> impl IntoView {
    view! {
        <CarouselPrevPrimitive>
            "‹"
        </CarouselPrevPrimitive>
    }
}

#[component]
pub fn CarouselNext() -> impl IntoView {
    view! {
        <CarouselNextPrimitive>
            "›"
        </CarouselNextPrimitive>
    }
}

#[component]
pub fn CarouselIndicators() -> impl IntoView {
    view! {
        <CarouselIndicatorsPrimitive>
            // Indicators são gerados dinamicamente pelo behavior
        </CarouselIndicatorsPrimitive>
    }
}
