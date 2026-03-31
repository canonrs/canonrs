# Textarea

id: textarea
label: Textarea
family: input
category: Form
intent: Capture multi-line text from user
description: Multi-line text input
composable: false
capabilities: Value, Disabled
required_parts: 
optional_parts: 
tags: textarea, multiline, comment, description, area
keywords: 
pain: Textarea misses required, readonly and aria attributes consistency
promise: All form states mapped directly to DOM and ARIA
why: TextareaPrimitive encodes disabled, readonly and required into both DOM and ARIA attributes. Label and description linkage is explicit. This guarantees accessible multi-line input behavior.
rules: CR-001, CR-004
use_cases: comments, descriptions
related: input, input_group, input_otp, field, label, checkbox, form_error_summary

## before
// ❌ Typical
view! {
  <textarea placeholder="Type..."></textarea>
}

## after
// ✅ CanonRS
view! {
  <Textarea placeholder="Type..." required=true />
}
