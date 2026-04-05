use leptos::prelude::*;
use super::collapsible_island::{CollapsibleIsland, CollapsibleTriggerIsland, CollapsibleContentIsland};

#[component]
pub fn CollapsibleShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <CollapsibleIsland>
                    <CollapsibleTriggerIsland>"Toggle details"</CollapsibleTriggerIsland>
                    <CollapsibleContentIsland>
                        "Hidden content revealed on toggle. State governed by signal."
                    </CollapsibleContentIsland>
                </CollapsibleIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Single toggle — open/close state via signal. SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Initially open"</span>
                <div data-rs-showcase-preview-row="">
                    <CollapsibleIsland open=true>
                        <CollapsibleTriggerIsland>"Advanced options"</CollapsibleTriggerIsland>
                        <CollapsibleContentIsland>
                            "These options are visible by default because open=true was passed."
                        </CollapsibleContentIsland>
                    </CollapsibleIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Nested"</span>
                <div data-rs-showcase-preview-row="">
                    <CollapsibleIsland>
                        <CollapsibleTriggerIsland>"Parent"</CollapsibleTriggerIsland>
                        <CollapsibleContentIsland>
                            <CollapsibleIsland>
                                <CollapsibleTriggerIsland>"Child"</CollapsibleTriggerIsland>
                                <CollapsibleContentIsland>"Nested collapsible content."</CollapsibleContentIsland>
                            </CollapsibleIsland>
                        </CollapsibleContentIsland>
                    </CollapsibleIsland>
                </div>
            </div>
        </div>
    }
}
