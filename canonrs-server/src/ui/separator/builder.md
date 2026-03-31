# Separator

id: separator
label: Separator
family: layout
category: Layout
intent: Visually divide content sections
description: Visual divider line
composable: false
capabilities: Orientation
required_parts: 
optional_parts: 
tags: separator, divider, line, hr, section
keywords: 
pain: Dividers lack semantic meaning and accessibility roles
promise: Separator semantics enforced via orientation and role contract
why: SeparatorPrimitive encodes orientation and decorative behavior. ARIA roles are derived automatically. This guarantees semantic and accessible separators.
rules: CR-001, CR-004
use_cases: layout separation, menus
related: card, resizable, scroll_area, aspect_ratio, page_header, toolbar

## before
// ❌ Typical
view! {
  <hr />
}

## after
// ✅ CanonRS
view! {
  <Separator />
}
