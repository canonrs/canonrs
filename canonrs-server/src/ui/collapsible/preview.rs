use leptos::prelude::*;
use super::collapsible_boundary::{Collapsible, CollapsibleTrigger, CollapsibleContent};

#[component]
pub fn CollapsibleShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Collapsible>
                    <CollapsibleTrigger>"Toggle details"</CollapsibleTrigger>
                    <CollapsibleContent>
                        "Hidden content revealed on toggle. State governed by signal."
                    </CollapsibleContent>
                </Collapsible>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Single toggle — open/close state via signal. SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Initially open"</span>
                <div data-rs-showcase-preview-row="">
                    <Collapsible open=true>
                        <CollapsibleTrigger>"Advanced options"</CollapsibleTrigger>
                        <CollapsibleContent>
                            "These options are visible by default because open=true was passed."
                        </CollapsibleContent>
                    </Collapsible>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Nested"</span>
                <div data-rs-showcase-preview-row="">
                    <Collapsible>
                        <CollapsibleTrigger>"Parent"</CollapsibleTrigger>
                        <CollapsibleContent>
                            <Collapsible>
                                <CollapsibleTrigger>"Child"</CollapsibleTrigger>
                                <CollapsibleContent>"Nested collapsible content."</CollapsibleContent>
                            </Collapsible>
                        </CollapsibleContent>
                    </Collapsible>
                </div>
            </div>
        </div>
    }
}
