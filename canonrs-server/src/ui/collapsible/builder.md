# Collapsible

id: collapsible
label: Collapsible
family: layout
category: Navigation
intent: Show and hide content sections
description: Collapsible section
composable: true
capabilities: OpenClose
required_parts: CollapsibleTrigger, CollapsibleContent
optional_parts: 
tags: collapsible, collapse, expand, hide, toggle
keywords: 
pain: Collapsible sections lose sync between trigger and content state
promise: Trigger and content state always synchronized via shared visibility state
why: CollapsiblePrimitive shares VisibilityState across trigger and content. This ensures both reflect the same open/closed state. The contract eliminates manual sync logic.
rules: CR-001, CR-004
use_cases: faq sections, expandable panels
related: 

## before
// ❌ Typical
view! {
  <button on:click=toggle>"Toggle"</button>
  {if open { view! { <div>"Content"</div> } }}
}

## after
// ✅ CanonRS
view! {
  <Collapsible>
    <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
    <CollapsibleContent>"Content"</CollapsibleContent>
  </Collapsible>
}
