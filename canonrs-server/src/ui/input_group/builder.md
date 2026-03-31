# Input Group

id: input-group
label: Input Group
family: utility
category: Form
intent: Group input with prefix or suffix elements
description: Input group with addons
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: input-group, prefix, suffix, addon, field
keywords: 
pain: Input addons break alignment and border radius consistency
promise: Grouped inputs maintain consistent structure and visual merging
why: InputGroupPrimitive encodes merge-radius behavior via ActivityState. This ensures inputs and addons render as a unified control. It prevents layout inconsistencies across grouped inputs.
rules: CR-001, CR-004
use_cases: email fields, currency inputs
related: input, input_otp, textarea, field, label, checkbox, form_error_summary

## before
// ❌ Typical
view! {
  <div class="input-group">
    <span>@</span>
    <input />
  </div>
}

## after
// ✅ CanonRS
view! {
  <InputGroup merge_radius=true>
    <span data-rs-input-group-addon="">"@"</span>
  </InputGroup>
}
