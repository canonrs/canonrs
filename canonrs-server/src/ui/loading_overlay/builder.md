# Loading Overlay

id: loading-overlay
label: Loading Overlay
family: utility
category: Display
intent: Block UI during async operations
description: Full loading overlay
composable: false
capabilities: OpenClose
required_parts: 
optional_parts: 
tags: loading-overlay, loading, overlay, wait, spinner, block
keywords: 
pain: Loading states require manual visibility and aria synchronization
promise: Loading visibility and aria-busy managed automatically
why: LoadingOverlayPrimitive maps LoadingState to visibility and ARIA attributes. Overlay visibility is derived automatically. This guarantees consistent loading feedback without manual logic.
rules: CR-001, CR-004
use_cases: async operations, page blocking
related: spinner, skeleton, pulse, doc_progress

## before
// ❌ Typical
view! {
  {if loading { view! { <div class="overlay">"Loading"</div> } }}
}

## after
// ✅ CanonRS
view! {
  <LoadingOverlay state=LoadingState::Loading>
    "Content"
  </LoadingOverlay>
}
