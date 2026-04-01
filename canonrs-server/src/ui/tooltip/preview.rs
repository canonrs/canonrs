use leptos::prelude::*;
use super::tooltip_ui::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent};
use canonrs_core::primitives::TooltipSide;

#[component]
pub fn TooltipShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TooltipProvider>
                    <Tooltip>
                        <TooltipTrigger tooltip_id="tt-1".to_string()>
                            <span data-rs-button="" data-rs-variant="outline">"Hover me"</span>
                        </TooltipTrigger>
                        <TooltipContent tooltip_id="tt-1".to_string()>"This is a tooltip"</TooltipContent>
                    </Tooltip>
                </TooltipProvider>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger-content relationship enforced without manual wiring."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;gap:1rem;flex-wrap:wrap;">
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger tooltip_id="tt-top".to_string()>
                                <span data-rs-button="" data-rs-variant="outline">"Top"</span>
                            </TooltipTrigger>
                            <TooltipContent tooltip_id="tt-top".to_string() side=TooltipSide::Top>"Top tooltip"</TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger tooltip_id="tt-bottom".to_string()>
                                <span data-rs-button="" data-rs-variant="outline">"Bottom"</span>
                            </TooltipTrigger>
                            <TooltipContent tooltip_id="tt-bottom".to_string() side=TooltipSide::Bottom>"Bottom tooltip"</TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger tooltip_id="tt-left".to_string()>
                                <span data-rs-button="" data-rs-variant="outline">"Left"</span>
                            </TooltipTrigger>
                            <TooltipContent tooltip_id="tt-left".to_string() side=TooltipSide::Left>"Left tooltip"</TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                    <TooltipProvider>
                        <Tooltip>
                            <TooltipTrigger tooltip_id="tt-right".to_string()>
                                <span data-rs-button="" data-rs-variant="outline">"Right"</span>
                            </TooltipTrigger>
                            <TooltipContent tooltip_id="tt-right".to_string() side=TooltipSide::Right>"Right tooltip"</TooltipContent>
                        </Tooltip>
                    </TooltipProvider>
                </div>
            </div>
        </div>
    }
}
