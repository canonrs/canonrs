use leptos::prelude::*;
use super::hover_card_boundary::{HoverCard, HoverCardTrigger, HoverCardContent};
use canonrs_core::primitives::HoverCardSide;

#[component]
pub fn HoverCardShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <HoverCard>
                    <HoverCardTrigger>"Hover me"</HoverCardTrigger>
                    <HoverCardContent side=HoverCardSide::Top>
                        <p>"This card appears on hover."</p>
                    </HoverCardContent>
                </HoverCard>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "HoverCard appears on mouse enter and dismisses on mouse leave."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <div data-rs-showcase-preview-row="">
                    <HoverCard>
                        <HoverCardTrigger>"Top"</HoverCardTrigger>
                        <HoverCardContent side=HoverCardSide::Top>"Opens above"</HoverCardContent>
                    </HoverCard>
                    <HoverCard>
                        <HoverCardTrigger>"Bottom"</HoverCardTrigger>
                        <HoverCardContent side=HoverCardSide::Bottom>"Opens below"</HoverCardContent>
                    </HoverCard>
                    <HoverCard>
                        <HoverCardTrigger>"Right"</HoverCardTrigger>
                        <HoverCardContent side=HoverCardSide::Right>"Opens right"</HoverCardContent>
                    </HoverCard>
                </div>
            </div>
        </div>
    }
}
