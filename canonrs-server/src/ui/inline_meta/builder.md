# Inline Meta

id: inline-meta
label: Inline Meta
family: family-f-data
category: Display
intent: Display structured metadata inline (label + value pairs)
description: Inline metadata display for stats, versions, dates and counts
composable: true
capabilities: 
required_parts: 
optional_parts: InlineMetaLabel, InlineMetaValue
tags: inline-meta, meta, label, value, stat, info, display
keywords: 
pain: Inline metadata mixes labels and values without structure
promise: Metadata pairs structured and consistently rendered
why: InlineMetaPrimitive separates label and value into explicit primitives. This enforces consistent layout and semantics. It prevents ad-hoc inline data rendering.
rules: CR-001, CR-004
use_cases: stats display, metadata labels
related: avatar, icon, logo, code_block, markdown, chart, stat, kbd


file: inline_meta_ui.css
tokens: inline-meta-*, space-*, font-*
foundation: spacing, typography
states: 
island: inline_meta_island.rs

## before
// ❌ Typical
view! {
  <span>"Rules: 284"</span>
}

## after
// ✅ CanonRS
view! {
  <InlineMeta>
    <InlineMetaLabel>"Rules"</InlineMetaLabel>
    <InlineMetaValue>"284"</InlineMetaValue>
  </InlineMeta>
}
