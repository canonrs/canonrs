# Switch

id: switch
label: Switch
family: family-c-forms
category: Form
intent: Toggle between on and off states
description: Toggle switch on off
composable: false
capabilities: Disabled
required_parts: 
optional_parts: 
tags: switch, toggle, on, off, activate
keywords: 
pain: Toggle inputs desync visual state and checked value
promise: Toggle state mapped directly to DOM and interaction state
why: SwitchPrimitive maps SelectionState to checked and data attributes. Disabled state is enforced consistently. This guarantees reliable toggle behavior.
rules: CR-001, CR-004
use_cases: settings, toggles
related: toggle, toggle_group


file: switch_ui.css
tokens: switch-*, size-*, radius-*, motion-*
foundation: size, radius, motion
states: on, off, disabled
island: switch_island.rs

pillar: toggle

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift, Island Architecture

## before
// ❌ Typical
view! {
  <input type="checkbox" checked />
}

## after
// ✅ CanonRS
view! {
  <Switch checked=true>"On"</Switch>
}
