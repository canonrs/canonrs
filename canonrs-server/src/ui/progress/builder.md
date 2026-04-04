# Progress

id: progress
label: Progress
family: family-e-feedback
category: Feedback
intent: Show completion of a task
description: Progress bar indicator
composable: true
capabilities: Value
required_parts: ProgressIndicator
optional_parts: 
tags: progress, bar, loading, percentage, completion
keywords: 
pain: Progress bars overflow or misreport values beyond valid range
promise: Progress value always clamped and ARIA-compliant
why: ProgressPrimitive clamps value between 0–100 and maps it to aria-valuenow. Indicator movement is derived from this value. This guarantees consistent visual and accessibility behavior.
rules: CR-001, CR-004
use_cases: file upload, task completion
related: spinner, skeleton, pulse, loading_overlay, doc_progress


file: progress_ui.css
tokens: progress-*, size-*, radius-*, motion-*
foundation: size, radius, motion
states: loading
island: progress_island.rs

## before
// ❌ Typical
view! {
  <div class="progress" style="width:150%"></div>
}

## after
// ✅ CanonRS
view! {
  <Progress value=75.0 />
}
