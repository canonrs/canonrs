# Button

id: button
label: Button
family: interactive
category: Action
intent: Trigger an action or event
description: Action button with variant and size
composable: false
capabilities: Disabled
required_parts: 
optional_parts: 
tags: button, action, submit, click, cta
keywords: button rust leptos, ssr safe button leptos, canonical button component rust, governed ui button
pain: Buttons rely on string classes causing inconsistent variants and states
promise: Variant and size enforced at compile-time via enums
why: ButtonVariant and ButtonSize define allowed visual and behavioral states. The primitive encodes these into data attributes, ensuring consistent rendering. This eliminates invalid combinations and style drift.
rules: CR-001, CR-004, CR-148
use_cases: form submit, cta actions
related: button_group, icon_button, copy_button, link

## before
// ❌ Typical
view! {
  <button class="btn btn-primary btn-md">"Submit"</button>
}

## after
// ✅ CanonRS
view! {
  <Button variant=ButtonVariant::Primary size=ButtonSize::Md>
    "Submit"
  </Button>
}
