# Accordion

id: accordion
label: Accordion
family: layout
category: Navigation
intent: Expand and collapse content sections
description: Expandable accordion sections
composable: true
capabilities: OpenClose, Multiple
required_parts: AccordionItem, AccordionTrigger, AccordionContent
optional_parts: 
tags: accordion, collapsible, expand, sections, faq
keywords: 
pain: Accordion allows invalid multi-open logic without enforced selection mode
promise: Selection mode enforced via type, preventing invalid open states
why: AccordionSelection defines whether single or multiple items can be open. The primitive encodes state and behavior in data-rs attributes, ensuring consistent disclosure logic. VisibilityState guarantees SSR-safe open/closed state without runtime drift.
rules: CR-001, CR-004
use_cases: faq sections, settings panels
related: collapsible

## before
// ❌ Typical
view! {
  <div class="accordion">
    <button on:click=move |_| toggle(1)>"Item 1"</button>
    {if open == 1 { view! { <div>"Content"</div> } } else { view! {} }}
  </div>
}

## after
// ✅ CanonRS
view! {
  <Accordion selection=AccordionSelection::Single>
    <AccordionItem>
      <AccordionTrigger>"Item 1"</AccordionTrigger>
      <AccordionContent>"Content"</AccordionContent>
    </AccordionItem>
  </Accordion>
}
