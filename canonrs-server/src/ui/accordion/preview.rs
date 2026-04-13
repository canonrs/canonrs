use leptos::prelude::*;
use super::accordion_boundary::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_core::primitives::AccordionSelection;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn AccordionShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Accordion>
                <AccordionItem>
                    <AccordionTrigger>"What is CanonRS?"</AccordionTrigger>
                    <AccordionContent>"CanonRS is a design system built in Rust and Leptos with a 3-layer architecture."</AccordionContent>
                </AccordionItem>
                <AccordionItem>
                    <AccordionTrigger>"How does it work?"</AccordionTrigger>
                    <AccordionContent>"Primitives define structure. Behaviors add interactivity. UI components compose both."</AccordionContent>
                </AccordionItem>
                <AccordionItem>
                    <AccordionTrigger>"Is SSR supported?"</AccordionTrigger>
                    <AccordionContent>"Yes. All state is defined at the primitive level via data-rs-state."</AccordionContent>
                </AccordionItem>
            </Accordion>
            <p data-rs-showcase-preview-anchor="">
                "Open/close state governed by DOM — single or multiple selection."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Multiple selection"</span>
                <Accordion selection=AccordionSelection::Multiple>
                    <AccordionItem>
                        <AccordionTrigger>"Section A"</AccordionTrigger>
                        <AccordionContent>"Content for section A."</AccordionContent>
                    </AccordionItem>
                    <AccordionItem>
                        <AccordionTrigger>"Section B"</AccordionTrigger>
                        <AccordionContent>"Content for section B."</AccordionContent>
                    </AccordionItem>
                </Accordion>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Non-collapsible"</span>
                <Accordion collapsible=false>
                    <AccordionItem>
                        <AccordionTrigger>"Always one open"</AccordionTrigger>
                        <AccordionContent>"This accordion always keeps one item open."</AccordionContent>
                    </AccordionItem>
                    <AccordionItem>
                        <AccordionTrigger>"Second item"</AccordionTrigger>
                        <AccordionContent>"Click to switch — cannot close all."</AccordionContent>
                    </AccordionItem>
                </Accordion>
            </Stack>
        </Stack>
    }
}
