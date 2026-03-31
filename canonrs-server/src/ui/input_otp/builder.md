# OTP Input

id: input-otp
label: OTP Input
family: utility
category: Form
intent: Capture one-time password codes
description: One-time password input
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: input-otp, otp, code, verification, sms, token, pin
keywords: 
pain: OTP inputs require complex state sync across multiple fields
promise: OTP slots and active state managed automatically
why: InputOtp distributes value across slots using ActivityState. Each slot reflects position and focus without manual logic. This guarantees synchronized UI and input state.
rules: CR-001, CR-004
use_cases: 2FA codes, verification inputs
related: input, input_group, textarea, field, label, checkbox, form_error_summary

## before
// ❌ Typical
// multiple inputs with manual focus handling

## after
// ✅ CanonRS
view! {
  <InputOtp length=6 />
}
