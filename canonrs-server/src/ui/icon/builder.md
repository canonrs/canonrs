# Icon

id: icon
label: Icon
family: family-f-data
category: Display
intent: Display an SVG icon
description: SVG icon display
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: icon, svg, symbol, visual
keywords: 
pain: Icons lack consistent sizing and variant handling
promise: Icon size and variant enforced via typed enums
why: Icon component encodes size and variant through enums. These map directly to data attributes. This guarantees consistent rendering and eliminates class-based inconsistencies.
rules: CR-001, CR-004
use_cases: buttons, status indicators
related: avatar, logo, code_block, markdown, chart, stat, inline_meta, kbd


file: icon_ui.css
tokens: icon-*, size-*
foundation: size, motion
states: 
island: icon_island.rs

## before
// ❌ Typical
view! {
  <svg class="icon-lg">"★"</svg>
}

## after
// ✅ CanonRS
view! {
  <Icon size=IconSize::Md>"★"</Icon>
}
