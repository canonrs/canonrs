# Status Dot

id: status-dot
label: Status Dot
family: family-e-feedback
category: Display
intent: Indicate user presence or availability
description: Status indicator dot
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: status-dot, status, indicator, online, offline, active
keywords: 
pain: Status indicators mix semantic feedback with presence states
promise: Presence states strictly separated from semantic feedback
why: StatusDotVariant encodes only presence states like online or busy. ARIA labels are derived automatically. This guarantees correct semantic usage.
rules: CR-001, CR-004
use_cases: user presence, chat apps
related: toast, alert, banner, callout, inline_notice


file: status_dot_ui.css
tokens: status-dot-*, size-*, radius-*
foundation: size, radius, motion
states: active, inactive
island: status_dot_island.rs

pillar: feedback

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift

## before
// ❌ Typical
view! {
  <span class="green-dot"></span>
}

## after
// ✅ CanonRS
view! {
  <StatusDot variant=StatusDotVariant::Online />
}
