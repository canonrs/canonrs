# Switch

id: switch
label: Switch
family: input
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
related: toggle_group

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
