# Radio

id: radio
label: Radio
family: input
category: Form
intent: Select one option from a group
description: Radio button input
composable: true
capabilities: Disabled
required_parts: RadioGroup
optional_parts: 
tags: radio, choice, exclusive, selection
keywords: 
pain: Radio inputs desync checked state and accessibility attributes
promise: Selection state mapped directly to DOM and ARIA
why: RadioPrimitive maps SelectionState to checked and aria attributes. Disabled state is also enforced structurally. This guarantees consistent selection behavior.
rules: CR-001, CR-004
use_cases: forms, single choice inputs
related: combobox, radio_group, color_picker, slider

## before
// ❌ Typical
view! {
  <input type="radio" checked />
}

## after
// ✅ CanonRS
view! {
  <Radio value="a" name="group" checked=true>"Option"</Radio>
}
