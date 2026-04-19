use leptos::prelude::*;
use super::hover_card_boundary::{HoverCard, HoverCardTrigger, HoverCardContent};
use canonrs_core::primitives::HoverCardSide;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn HoverCardShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <HoverCard>
                <HoverCardTrigger>"Hover me"</HoverCardTrigger>
                <HoverCardContent side=HoverCardSide::Bottom>
                    <p>"This card appears on hover."</p>
                </HoverCardContent>
            </HoverCard>
            <p data-rs-showcase-preview-anchor="">
                "HoverCard appears on mouse enter and dismisses on mouse leave."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
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
                </Stack>
            </Stack>
        </Stack>
    }
}
