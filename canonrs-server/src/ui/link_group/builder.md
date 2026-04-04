# Link Group

id: link-group
label: Link Group
family: family-d-navigation
category: Navigation
intent: Semantic group of navigation links with optional label
description: Wrapper that organizes multiple NavItems into a labeled navigation group
composable: true
capabilities: Orientation, Disabled
required_parts: 
optional_parts: 
tags: link-group, nav-group, navigation, links, footer, sidebar, grouped
keywords: 
pain: Navigation links lack grouping semantics and structural consistency
promise: Grouped navigation structured with direction and labeling contract
why: LinkGroup uses NavigationGroup primitives to enforce grouping and labeling. Direction is encoded via enum. This guarantees consistent navigation structure across layouts.
rules: CR-001, CR-004
use_cases: sidebars, footers
related: navigation_menu, sidebar, nav_item, breadcrumb, pagination


file: link_group_ui.css
tokens: link-group-*, space-*, font-*
foundation: spacing, typography
states: active
island: link_group_island.rs

pillar: navigation

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift

## before
// ❌ Typical
view! {
  <div>
    <a>"A"</a>
    <a>"B"</a>
  </div>
}

## after
// ✅ CanonRS
view! {
  <LinkGroup>
    <NavItem label="A" href="/a" />
  </LinkGroup>
}
