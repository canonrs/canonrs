# Table of Contents

id: table-of-contents
label: Table of Contents
family: family-d-navigation
category: Navigation
intent: Navigate document sections via anchors
description: Document table of contents
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: table-of-contents, index, summary, anchors, navigation, document
keywords: 
pain: TOC structures inconsistent and hard to sync with document hierarchy
promise: TOC hierarchy and state derived from structured data model
why: TocPrimitive encodes mode and item states with hierarchical structure. SSR rendering ensures deterministic output. This guarantees consistent navigation.
rules: CR-001, CR-004
use_cases: docs navigation, long pages
related: collapsible


file: table_of_contents_ui.css
tokens: toc-*, space-*, font-*, radius-*, motion-*
foundation: spacing, radius, motion, typography
states: active, open, closed
island: table_of_contents_island.rs

## before
// ❌ Typical
view! {
  <ul>
    <li><a href="#a">"A"</a></li>
  </ul>
}

## after
// ✅ CanonRS
view! {
  <TableOfContents items=items />
}
