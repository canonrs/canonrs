use leptos::prelude::*;
use super::boundary::{
    TooltipProvider, Tooltip, TooltipTrigger, TooltipContent,
};
use canonrs_core::primitives::TooltipSide;

#[component]
pub fn TooltipShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger tooltip_id="tt-1">"Hover me"</TooltipTrigger>
                        <TooltipContent tooltip_id="tt-1">"Tooltip content"</TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tooltip appears on hover and focus with configurable delay."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <div data-rs-showcase-preview-row="">
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger tooltip_id="tt-top">"Top"</TooltipTrigger>
                            <TooltipContent side=TooltipSide::Top tooltip_id="tt-top">"Opens above"</TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger tooltip_id="tt-bottom">"Bottom"</TooltipTrigger>
                            <TooltipContent side=TooltipSide::Bottom tooltip_id="tt-bottom">"Opens below"</TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger tooltip_id="tt-right">"Right"</TooltipTrigger>
                            <TooltipContent side=TooltipSide::Right tooltip_id="tt-right">"Opens right"</TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </div>
            </div>
        </div>
    }
}
