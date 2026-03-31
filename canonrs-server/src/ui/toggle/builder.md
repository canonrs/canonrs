# Toggle

id: toggle
label: Toggle
family: input
category: Form
intent: Toggle a pressed state
description: Toggle button
composable: false
capabilities: Pressed, Disabled
required_parts: 
optional_parts: 
tags: toggle, activate, button, on, off, press
keywords: 
pain: Toggle buttons desync pressed state and visual representation
promise: Pressed state mapped directly to DOM and interaction state
why: TogglePrimitive maps ToggleState to data-rs-state and checkbox checked state. Disabled and aria-label are enforced. This guarantees consistent toggle behavior.
rules: CR-001, CR-004
use_cases: formatting tools, feature toggles
related: switch, toggle_group

## before
// ❌ Typical
view! {
  <button class="active">"On"</button>
}

## after
// ✅ CanonRS
view! {
  <Toggle pressed=true>"On"</Toggle>
}
