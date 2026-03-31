# Status Dot

id: status-dot
label: Status Dot
family: data_display
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
related: toast, alert, banner, callout, inline_notice, inline_meta, badge

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
