# Label

id: label
label: Label
family: family-c-forms
category: Form
intent: Associate a label with a form control
description: Form label component
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: label, form, accessibility, input
keywords: 
pain: Labels not correctly associated with inputs, breaking accessibility
promise: Label-to-input association enforced via explicit html_for contract
why: LabelPrimitive ensures proper for/id mapping and required state. ARIA attributes are derived automatically. This guarantees accessible labeling without manual wiring.
rules: CR-001, CR-004
use_cases: forms, inputs
related: form, input, input_group, input_otp, textarea, field, checkbox, form_error_summary


file: label_ui.css
tokens: label-*, font-*
foundation: typography
states: 
island: label_island.rs

pillar: form

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift

## before
// ❌ Typical
view! {
  <label>"Email"</label>
  <input />
}

## after
// ✅ CanonRS
view! {
  <Label for_id="email">"Email"</Label>
}
