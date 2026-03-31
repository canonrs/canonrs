# Button Group

id: button-group
label: Button Group
family: utility
category: Action
intent: Group related action buttons together
description: Group of action buttons
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: button-group, buttons, group, actions, multiple
keywords: 
pain: Grouped buttons lose semantic grouping and accessibility context
promise: Group semantics and attachment enforced via component contract
why: ButtonGroupPrimitive defines role="group" and controlled attachment state via ToggleState. This ensures grouped actions are treated as a single logical unit. The contract guarantees consistent accessibility and visual cohesion.
rules: CR-001, CR-004
use_cases: toolbar actions, segmented controls
related: button, icon_button, copy_button, link

## before
// ❌ Typical
view! {
  <div class="btn-group">
    <button>"Left"</button>
    <button>"Right"</button>
  </div>
}

## after
// ✅ CanonRS
view! {
  <ButtonGroup>
    <Button>"Left"</Button>
    <Button>"Right"</Button>
  </ButtonGroup>
}
