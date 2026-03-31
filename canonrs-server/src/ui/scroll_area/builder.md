# Scroll Area

id: scroll-area
label: Scroll Area
family: layout
category: Layout
intent: Scrollable container with custom scrollbar
description: Scrollable area container
composable: false
capabilities: Overflow
required_parts: 
optional_parts: 
tags: scroll-area, scroll, overflow, container, long-list
keywords: 
pain: Custom scroll containers break keyboard navigation and accessibility
promise: Scroll behavior and accessibility enforced via structured container
why: ScrollAreaPrimitive defines viewport, scrollbars and orientation explicitly. ARIA roles and keyboard access are enforced. This guarantees accessible scrolling behavior.
rules: CR-001, CR-004
use_cases: long lists, logs
related: card, resizable, aspect_ratio, page_header, toolbar, separator

## before
// ❌ Typical
view! {
  <div style="overflow:auto;height:200px"></div>
}

## after
// ✅ CanonRS
view! {
  <ScrollArea>
    <div>"Content"</div>
  </ScrollArea>
}
