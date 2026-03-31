# Skeleton

id: skeleton
label: Skeleton
family: feedback
category: Feedback
intent: Show placeholder while content loads
description: Loading skeleton placeholder
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: skeleton, loading, placeholder, shimmer
keywords: 
pain: Loading placeholders inconsistent and lack accessibility semantics
promise: Skeleton state and variant standardized via structure
why: SkeletonPrimitive encodes variant and loading state with aria-busy. This guarantees consistent placeholder rendering and accessibility feedback.
rules: CR-001, CR-004
use_cases: loading states, content placeholders
related: spinner, pulse, loading_overlay, doc_progress

## before
// ❌ Typical
view! {
  <div class="skeleton"></div>
}

## after
// ✅ CanonRS
view! {
  <Skeleton variant=SkeletonVariant::Text />
}
