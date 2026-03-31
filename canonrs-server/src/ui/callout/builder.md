# Callout

id: callout
label: Callout
family: feedback
category: Feedback
intent: Highlight important contextual information
description: Callout info box
composable: true
capabilities: 
required_parts: 
optional_parts: CalloutIcon, CalloutTitle, CalloutDescription
tags: callout, highlight, info, note, warning, tip
keywords: 
pain: Callouts use inconsistent roles and lack semantic intent
promise: Semantic role and urgency enforced via variant
why: CalloutVariant determines role and aria-live behavior. The primitive encodes these semantics directly into the DOM. This guarantees consistent accessibility and meaning across all callouts.
rules: CR-001, CR-004
use_cases: tips, warnings
related: toast, alert, banner, inline_notice, status_dot, inline_meta, badge

## before
// ❌ Typical
view! {
  <div class="callout warning">"Be careful"</div>
}

## after
// ✅ CanonRS
view! {
  <Callout variant=CalloutVariant::Warning>
    <CalloutTitle>"Warning"</CalloutTitle>
    <CalloutDescription>"Be careful"</CalloutDescription>
  </Callout>
}
