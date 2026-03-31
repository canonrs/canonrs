# Empty State

id: empty-state
label: Empty State
family: feedback
category: Feedback
intent: Display when no content is available
description: Empty state placeholder
composable: true
capabilities: 
required_parts: 
optional_parts: EmptyStateIcon, EmptyStateTitle, EmptyStateDescription, EmptyStateAction
tags: empty-state, empty, no-data, placeholder, zero-state
keywords: 
pain: Empty states are inconsistent and lack semantic meaning
promise: Empty state intent and variant enforced via contract
why: EmptyStatePrimitive encodes variant and ARIA role="status". Variants ensure consistent messaging patterns. This guarantees predictable feedback across all empty states.
rules: CR-001, CR-004
use_cases: no results, empty dashboards
related: error_state

## before
// ❌ Typical
view! {
  <div>"No data"</div>
}

## after
// ✅ CanonRS
view! {
  <EmptyState>
    <EmptyStateTitle>"No data"</EmptyStateTitle>
  </EmptyState>
}
