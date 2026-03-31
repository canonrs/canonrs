# Icon Button

id: icon-button
label: Icon Button
family: interactive
category: Action
intent: Trigger an action with an icon button
description: Button with icon only
composable: false
capabilities: Disabled
required_parts: 
optional_parts: 
tags: icon-button, icon, action, close, delete, edit
keywords: 
pain: Icon buttons miss accessibility labels and loading state handling
promise: Accessibility and loading state enforced in button contract
why: IconButtonPrimitive requires aria-label and encodes loading and disabled states. ARIA attributes are derived automatically. This guarantees accessible and predictable behavior.
rules: CR-001, CR-004
use_cases: close actions, toolbar buttons
related: button, button_group, copy_button, link

## before
// ❌ Typical
view! {
  <button><svg /></button>
}

## after
// ✅ CanonRS
view! {
  <IconButton aria_label="Close">"×"</IconButton>
}
