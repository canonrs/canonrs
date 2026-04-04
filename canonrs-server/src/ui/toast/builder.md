# Toast

id: toast
label: Toast
family: family-e-feedback
category: Feedback
intent: Show brief non-blocking notifications
description: Toast notification message
composable: true
capabilities: OpenClose
required_parts: ToastViewport
optional_parts: ToastTitle, ToastDescription, ToastAction, ToastClose
tags: toast, notification, snackbar, message, success, error
keywords: 
pain: Notifications misuse urgency, role and lifecycle behavior
promise: Variant enforces correct role and aria-live automatically
why: ToastVariant defines role and aria-live behavior per type. Lifecycle and visibility are encoded structurally. This guarantees correct notification semantics and timing.
rules: CR-001, CR-004
use_cases: system notifications, user feedback
related: alert, banner, callout, inline_notice, status_dot


file: toast_ui.css
tokens: toast-*, space-*, radius-*, shadow-*, motion-*, font-*
foundation: spacing, radius, shadow, motion, typography
states: open, closed
island: toast_island.rs

pillar: feedback

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift, Island Architecture

## before
// ❌ Typical
view! {
  <div class="toast error">"Error"</div>
}

## after
// ✅ CanonRS
view! {
  <Toast variant=ToastVariant::Error>
    <ToastTitle>"Error"</ToastTitle>
  </Toast>
}
