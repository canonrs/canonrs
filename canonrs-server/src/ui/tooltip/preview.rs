use leptos::prelude::*;
use super::tooltip_island::{
    TooltipProviderIsland, TooltipIsland, TooltipTriggerIsland, TooltipContentIsland,
};
use canonrs_core::primitives::TooltipSide;

#[component]
pub fn TooltipShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TooltipProviderIsland>
                    <TooltipIsland>
                        <TooltipTriggerIsland tooltip_id="tt-1">"Hover me"</TooltipTriggerIsland>
                        <TooltipContentIsland tooltip_id="tt-1">"Tooltip content"</TooltipContentIsland>
                    </TooltipIsland>
                </TooltipProviderIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tooltip appears on hover and focus with configurable delay."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <div data-rs-showcase-preview-row="">
                    <TooltipProviderIsland>
                        <TooltipIsland>
                            <TooltipTriggerIsland tooltip_id="tt-top">"Top"</TooltipTriggerIsland>
                            <TooltipContentIsland side=TooltipSide::Top tooltip_id="tt-top">"Opens above"</TooltipContentIsland>
                        </TooltipIsland>
                    </TooltipProviderIsland>
                    <TooltipProviderIsland>
                        <TooltipIsland>
                            <TooltipTriggerIsland tooltip_id="tt-bottom">"Bottom"</TooltipTriggerIsland>
                            <TooltipContentIsland side=TooltipSide::Bottom tooltip_id="tt-bottom">"Opens below"</TooltipContentIsland>
                        </TooltipIsland>
                    </TooltipProviderIsland>
                    <TooltipProviderIsland>
                        <TooltipIsland>
                            <TooltipTriggerIsland tooltip_id="tt-right">"Right"</TooltipTriggerIsland>
                            <TooltipContentIsland side=TooltipSide::Right tooltip_id="tt-right">"Opens right"</TooltipContentIsland>
                        </TooltipIsland>
                    </TooltipProviderIsland>
                </div>
            </div>
        </div>
    }
}
