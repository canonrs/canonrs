# Toggle Group

id: toggle-group
label: Toggle Group
family: interactive
category: Action
intent: Group of toggle buttons with single or multiple selection
description: Group of toggle buttons
composable: true
capabilities: Multiple, Disabled
required_parts: 
optional_parts: 
tags: toggle-group, toggle, group, buttons, options, selection
keywords: 
pain: Grouped toggles lack exclusivity and shared disabled state
promise: Group behavior and selection mode enforced structurally
why: ToggleGroupPrimitive encodes multiple selection and disabled state at container level. Child toggles inherit behavior. This guarantees consistent grouped interactions.
rules: CR-001, CR-004
use_cases: toolbars, option groups
related: switch

## before
// ❌ Typical
view! {
  <div>
    <button>"A"</button>
    <button>"B"</button>
  </div>
}

## after
// ✅ CanonRS
view! {
  <ToggleGroup multiple=false>
    <Toggle>"A"</Toggle>
  </ToggleGroup>
}
