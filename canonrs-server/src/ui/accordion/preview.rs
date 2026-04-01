use leptos::prelude::*;
use super::accordion_ui::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
use canonrs_core::primitives::AccordionSelection;

#[component]
pub fn AccordionShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Accordion>
                    <AccordionItem>
                        <AccordionTrigger>"What is CanonRS?"</AccordionTrigger>
                        <AccordionContent>"CanonRS is a design system built in Rust and Leptos with a 3-layer architecture: primitives, behaviors, and UI components."</AccordionContent>
                    </AccordionItem>
                    <AccordionItem>
                        <AccordionTrigger>"How does it work?"</AccordionTrigger>
                        <AccordionContent>"Primitives define structure via semantic HTML and data-rs-* attributes. Behaviors add interactivity via WASM. UI components compose both layers."</AccordionContent>
                    </AccordionItem>
                    <AccordionItem>
                        <AccordionTrigger>"Is SSR supported?"</AccordionTrigger>
                        <AccordionContent>"Yes. All state is defined at the primitive level via data-rs-state, ensuring consistent SSR and hydration."</AccordionContent>
                    </AccordionItem>
                </Accordion>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Open/close state governed by data-rs-state — single or multiple selection."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Multiple selection"</span>
                <div data-rs-showcase-preview-row="">
                    <Accordion selection=AccordionSelection::Multiple>
                        <AccordionItem>
                            <AccordionTrigger>"Section A"</AccordionTrigger>
                            <AccordionContent>"Content for section A."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem>
                            <AccordionTrigger>"Section B"</AccordionTrigger>
                            <AccordionContent>"Content for section B."</AccordionContent>
                        </AccordionItem>
                        <AccordionItem>
                            <AccordionTrigger>"Section C"</AccordionTrigger>
                            <AccordionContent>"Content for section C."</AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Non-collapsible"</span>
                <div data-rs-showcase-preview-row="">
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
                </div>
            </div>
        </div>
    }
}
