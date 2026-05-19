#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::CarouselItemPrimitive;
use canonrs_core::meta::{ActivityState, VisibilityState};

#[component]
pub fn CarouselItem(
    #[prop(default = String::new())] class: String,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    let activity   = if active { ActivityState::Active   } else { ActivityState::Inactive };
    let visibility = if active { VisibilityState::Open   } else { VisibilityState::Closed };
    view! { <CarouselItemPrimitive class=class activity=activity visibility=visibility>{children.map(|c| c())}</CarouselItemPrimitive> }
}
