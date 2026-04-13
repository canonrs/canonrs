use leptos::prelude::*;
use super::collapsible_boundary::{Collapsible, CollapsibleTrigger, CollapsibleContent};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CollapsibleShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Collapsible>
                <CollapsibleTrigger>"Toggle details"</CollapsibleTrigger>
                <CollapsibleContent>
                    "Hidden content revealed on toggle. State governed by signal."
                </CollapsibleContent>
            </Collapsible>
            <p data-rs-showcase-preview-anchor="">
                "Single toggle — open/close state via signal. SSR-safe, hydration-safe."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Initially open"</span>
                <Collapsible open=true>
                    <CollapsibleTrigger>"Advanced options"</CollapsibleTrigger>
                    <CollapsibleContent>
                        "These options are visible by default because open=true was passed."
                    </CollapsibleContent>
                </Collapsible>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Nested"</span>
                <Collapsible>
                    <CollapsibleTrigger>"Parent"</CollapsibleTrigger>
                    <CollapsibleContent>
                        <Collapsible>
                            <CollapsibleTrigger>"Child"</CollapsibleTrigger>
                            <CollapsibleContent>"Nested collapsible content."</CollapsibleContent>
                        </Collapsible>
                    </CollapsibleContent>
                </Collapsible>
            </Stack>
        </Stack>
    }
}
