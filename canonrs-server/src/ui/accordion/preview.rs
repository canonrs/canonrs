use leptos::prelude::*;
use super::accordion_island::{AccordionIsland, AccordionIslandItem, AccordionSelectionMode};

fn faq_items() -> Vec<AccordionIslandItem> {
    vec![
        AccordionIslandItem { value: "a1".into(), trigger: "What is CanonRS?".into(), content: "CanonRS is a design system built in Rust and Leptos with a 3-layer architecture: primitives, behaviors, and UI components.".into(), disabled: false },
        AccordionIslandItem { value: "a2".into(), trigger: "How does it work?".into(), content: "Primitives define structure via semantic HTML and data-rs-* attributes. Behaviors add interactivity via WASM. UI components compose both layers.".into(), disabled: false },
        AccordionIslandItem { value: "a3".into(), trigger: "Is SSR supported?".into(), content: "Yes. All state is defined at the primitive level via data-rs-state, ensuring consistent SSR and hydration.".into(), disabled: false },
    ]
}

fn multi_items() -> Vec<AccordionIslandItem> {
    vec![
        AccordionIslandItem { value: "b1".into(), trigger: "Section A".into(), content: "Content for section A.".into(), disabled: false },
        AccordionIslandItem { value: "b2".into(), trigger: "Section B".into(), content: "Content for section B.".into(), disabled: false },
        AccordionIslandItem { value: "b3".into(), trigger: "Section C".into(), content: "Content for section C.".into(), disabled: false },
    ]
}

fn noncollapsible_items() -> Vec<AccordionIslandItem> {
    vec![
        AccordionIslandItem { value: "c1".into(), trigger: "Always one open".into(), content: "This accordion always keeps one item open.".into(), disabled: false },
        AccordionIslandItem { value: "c2".into(), trigger: "Second item".into(), content: "Click to switch — cannot close all.".into(), disabled: false },
    ]
}

#[component]
pub fn AccordionShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <AccordionIsland items=faq_items() />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Open/close state governed by DOM — single or multiple selection."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Multiple selection"</span>
                <div data-rs-showcase-preview-row="">
                    <AccordionIsland items=multi_items() selection=AccordionSelectionMode::Multiple />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Non-collapsible"</span>
                <div data-rs-showcase-preview-row="">
                    <AccordionIsland items=noncollapsible_items() collapsible=false />
                </div>
            </div>
        </div>
    }
}
