# Alert

id: alert
label: Alert
family: family-e-feedback
category: Feedback
intent: Display important static messages
description: Alert message box
composable: true
capabilities: 
required_parts: 
optional_parts: AlertTitle, AlertDescription
tags: alert, warning, info, message, error
keywords: 
pain: Alerts use wrong ARIA roles causing accessibility issues silently
promise: Correct ARIA role and live region enforced by variant
why: AlertVariant controls both semantic role and aria-live behavior. The primitive guarantees correct accessibility mapping at compile-time. This prevents incorrect alert semantics without relying on developer discipline.
rules: CR-001, CR-004
use_cases: error messages, status notifications
related: toast, banner, callout, inline_notice, status_dot


file: alert_ui.css
tokens: alert-*, space-*, radius-*, font-*, motion-*
foundation: spacing, radius, typography, motion
states: open, closed
island: alert_island.rs

pillar: feedback

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift

## before
// ❌ Typical
view! {
  <div class="alert alert-error">"Error occurred"</div>
}

## after
// ✅ CanonRS
view! {
  <Alert variant=AlertVariant::Destructive>
    <AlertTitle>"Error"</AlertTitle>
    <AlertDescription>"Error occurred"</AlertDescription>
  </Alert>
}
