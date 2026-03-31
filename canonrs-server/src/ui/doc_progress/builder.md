# Doc Progress

id: doc-progress
label: Doc Progress
family: utility
category: Display
intent: Indicate reading progress in a document
description: Document progress indicator
composable: 
capabilities: 
required_parts: 
optional_parts: 
tags: doc-progress, document, progress, reading
keywords: 
pain: Scroll progress indicators require manual scroll tracking logic
promise: Progress tracking injected automatically via behavior layer
why: DocProgressPrimitive exposes progress via data attributes and ARIA. Portal variant allows injection anywhere in layout. This guarantees consistent scroll tracking without custom JS.
rules: CR-001, CR-004
use_cases: docs reading, long articles
related: spinner, skeleton, pulse, loading_overlay

## before
// ❌ Typical
window.onscroll = () => updateProgress();

## after
// ✅ CanonRS
view! {
  <DocProgress />
}
