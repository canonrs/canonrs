# Resizable

id: resizable
label: Resizable
family: layout
category: Layout
intent: Split panels with draggable divider
description: Resizable panel component
composable: true
capabilities: Orientation, Resize
required_parts: ResizablePanel, ResizableHandle
optional_parts: 
tags: resizable, resize, panel, split, adjust
keywords: 
pain: Resizable panels break layout constraints and overflow limits
promise: Panel sizes constrained and behavior encoded structurally
why: ResizablePrimitive defines orientation and size limits via attributes. Panels and handles follow strict composition. This guarantees controlled resizing behavior.
rules: CR-001, CR-004
use_cases: editors, dashboards
related: card, scroll_area, aspect_ratio, page_header, toolbar, separator

## before
// ❌ Typical
view! {
  <div class="split">
    <div></div>
    <div></div>
  </div>
}

## after
// ✅ CanonRS
view! {
  <Resizable>
    <ResizablePanel />
    <ResizableHandle />
  </Resizable>
}
