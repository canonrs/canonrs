use leptos::prelude::*;
use super::hover_card_island::{HoverCardIsland, HoverCardTriggerIsland, HoverCardContentIsland};
use canonrs_core::primitives::HoverCardSide;

#[component]
pub fn HoverCardShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <HoverCardIsland>
                    <HoverCardTriggerIsland>"Hover me"</HoverCardTriggerIsland>
                    <HoverCardContentIsland side=HoverCardSide::Top>
                        <p>"This card appears on hover."</p>
                    </HoverCardContentIsland>
                </HoverCardIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "HoverCard appears on mouse enter and dismisses on mouse leave."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <div data-rs-showcase-preview-row="">
                    <HoverCardIsland>
                        <HoverCardTriggerIsland>"Top"</HoverCardTriggerIsland>
                        <HoverCardContentIsland side=HoverCardSide::Top>"Opens above"</HoverCardContentIsland>
                    </HoverCardIsland>
                    <HoverCardIsland>
                        <HoverCardTriggerIsland>"Bottom"</HoverCardTriggerIsland>
                        <HoverCardContentIsland side=HoverCardSide::Bottom>"Opens below"</HoverCardContentIsland>
                    </HoverCardIsland>
                    <HoverCardIsland>
                        <HoverCardTriggerIsland>"Right"</HoverCardTriggerIsland>
                        <HoverCardContentIsland side=HoverCardSide::Right>"Opens right"</HoverCardContentIsland>
                    </HoverCardIsland>
                </div>
            </div>
        </div>
    }
}
