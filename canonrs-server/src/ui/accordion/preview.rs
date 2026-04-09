use leptos::prelude::*;
use super::accordion_island::{AccordionIsland, AccordionItemIsland, AccordionTriggerIsland, AccordionContentIsland};
use canonrs_core::primitives::AccordionSelection;

#[component]
pub fn AccordionShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <AccordionIsland>
                    <AccordionItemIsland>
                        <AccordionTriggerIsland>"What is CanonRS?"</AccordionTriggerIsland>
                        <AccordionContentIsland>"CanonRS is a design system built in Rust and Leptos with a 3-layer architecture."</AccordionContentIsland>
                    </AccordionItemIsland>
                    <AccordionItemIsland>
                        <AccordionTriggerIsland>"How does it work?"</AccordionTriggerIsland>
                        <AccordionContentIsland>"Primitives define structure. Behaviors add interactivity. UI components compose both."</AccordionContentIsland>
                    </AccordionItemIsland>
                    <AccordionItemIsland>
                        <AccordionTriggerIsland>"Is SSR supported?"</AccordionTriggerIsland>
                        <AccordionContentIsland>"Yes. All state is defined at the primitive level via data-rs-state."</AccordionContentIsland>
                    </AccordionItemIsland>
                </AccordionIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Open/close state governed by DOM — single or multiple selection."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Multiple selection"</span>
                <div data-rs-showcase-preview-row="">
                    <AccordionIsland selection=AccordionSelection::Multiple>
                        <AccordionItemIsland>
                            <AccordionTriggerIsland>"Section A"</AccordionTriggerIsland>
                            <AccordionContentIsland>"Content for section A."</AccordionContentIsland>
                        </AccordionItemIsland>
                        <AccordionItemIsland>
                            <AccordionTriggerIsland>"Section B"</AccordionTriggerIsland>
                            <AccordionContentIsland>"Content for section B."</AccordionContentIsland>
                        </AccordionItemIsland>
                    </AccordionIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Non-collapsible"</span>
                <div data-rs-showcase-preview-row="">
                    <AccordionIsland collapsible=false>
                        <AccordionItemIsland>
                            <AccordionTriggerIsland>"Always one open"</AccordionTriggerIsland>
                            <AccordionContentIsland>"This accordion always keeps one item open."</AccordionContentIsland>
                        </AccordionItemIsland>
                        <AccordionItemIsland>
                            <AccordionTriggerIsland>"Second item"</AccordionTriggerIsland>
                            <AccordionContentIsland>"Click to switch — cannot close all."</AccordionContentIsland>
                        </AccordionItemIsland>
                    </AccordionIsland>
                </div>
            </div>
        </div>
    }
}
