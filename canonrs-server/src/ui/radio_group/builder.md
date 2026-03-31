# Radio Group

id: radio-group
label: Radio Group
family: input
category: Form
intent: Group radio buttons for single selection
description: Group of radio buttons
composable: false
capabilities: Disabled
required_parts: 
optional_parts: 
tags: radio-group, radio, group, options, alternatives, choice
keywords: 
pain: Radio groups lose exclusivity and accessibility grouping
promise: Group semantics and exclusivity enforced at container level
why: RadioGroupPrimitive enforces role="radiogroup" and shared disabled state. Items derive selection state consistently. This guarantees exclusive selection behavior.
rules: CR-001, CR-004
use_cases: settings, forms
related: combobox, radio, color_picker, slider

## before
// ❌ Typical
view! {
  <div>
    <input type="radio" name="a" />
    <input type="radio" name="a" />
  </div>
}

## after
// ✅ CanonRS
view! {
  <RadioGroup>
    <RadioGroupItem name="a" value="1">"One"</RadioGroupItem>
  </RadioGroup>
}
