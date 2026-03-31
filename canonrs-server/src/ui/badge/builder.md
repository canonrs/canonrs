# Badge

id: badge
label: Badge
family: data_display
category: Display
intent: Display status, count or label
description: Status badge label
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: badge, tag, status, label, notification
keywords: 
pain: Badges mix interactive and static behavior without clear intent
promise: Interactivity explicitly defined and enforced by type
why: BadgeInteractivity defines whether the badge is static or interactive. The primitive encodes this into data attributes, preventing misuse. This ensures consistent semantics and avoids accidental clickable badges.
rules: CR-001, CR-004
use_cases: status labels, notifications
related: toast, alert, banner, callout, inline_notice, status_dot, inline_meta

## before
// ❌ Typical
view! {
  <span class="badge clickable">"New"</span>
}

## after
// ✅ CanonRS
view! {
  <Badge interactivity=BadgeInteractivity::Static>
    "New"
  </Badge>
}
