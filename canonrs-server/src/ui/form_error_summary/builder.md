# Form Error Summary

id: form-error-summary
label: Form Error Summary
family: utility
category: Form
intent: Summarize form validation errors
description: Form validation error summary
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: form-error-summary, error, validation, form, summary
keywords: 
pain: Form errors scattered and not announced collectively
promise: All form errors announced together with structured summary
why: FormErrorSummaryPrimitive uses role="alert" with aria-atomic to ensure full error announcement. Errors are grouped and structured. This guarantees accessibility and visibility of all validation issues.
rules: CR-001, CR-004
use_cases: form validation, multi-field errors
related: input, input_group, input_otp, textarea, field, label, checkbox

## before
// ❌ Typical
view! {
  <div>"Error"</div>
}

## after
// ✅ CanonRS
view! {
  <FormErrorSummary errors=vec![] />
}
