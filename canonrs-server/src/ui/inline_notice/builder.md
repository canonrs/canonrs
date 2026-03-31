# Inline Notice

id: inline-notice
label: Inline Notice
family: feedback
category: Feedback
intent: Show inline contextual feedback
description: Inline notice message
composable: true
capabilities: 
required_parts: 
optional_parts: InlineNoticeIcon, InlineNoticeContent
tags: inline-notice, notice, warning, inline, message, info
keywords: 
pain: Inline messages use wrong ARIA roles and urgency levels
promise: Role and aria-live automatically enforced by variant
why: InlineNoticeVariant controls both semantic role and aria-live behavior. Error uses alert/assertive while others use status/polite. This guarantees correct urgency signaling without manual ARIA decisions.
rules: CR-001, CR-004
use_cases: form inline errors, contextual hints
related: toast, alert, banner, callout, status_dot, inline_meta, badge

## before
// ❌ Typical
view! {
  <div class="notice error">"Error"</div>
}

## after
// ✅ CanonRS
view! {
  <InlineNotice variant=InlineNoticeVariant::Error>
    <InlineNoticeContent>"Error"</InlineNoticeContent>
  </InlineNotice>
}
