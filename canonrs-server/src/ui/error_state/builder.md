# Error State

id: error-state
label: Error State
family: family-e-feedback
category: Feedback
intent: Display error condition to user
description: Error state display
composable: true
capabilities: 
required_parts: 
optional_parts: ErrorStateIcon, ErrorStateTitle, ErrorStateDescription, ErrorStateAction
tags: error-state, error, failure, problem, try-again
keywords: 
pain: Error messages inconsistent and not announced to assistive technologies
promise: Error feedback always announced and structurally consistent
why: ErrorStatePrimitive enforces role="status" with aria-live="assertive". This guarantees immediate announcement of critical errors. Structure ensures consistent composition of icon, title and actions.
rules: CR-001, CR-004
use_cases: api failures, form submission errors
related: empty_state, animate


file: error_state_ui.css
tokens: error-state-*, space-*, size-*, font-*
foundation: spacing, size, typography
states: error
island: error_state_island.rs

pillar: feedback_state

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift

## before
// ❌ Typical
view! {
  <div class="error">"Something went wrong"</div>
}

## after
// ✅ CanonRS
view! {
  <ErrorState>
    <ErrorStateTitle>"Error"</ErrorStateTitle>
  </ErrorState>
}
