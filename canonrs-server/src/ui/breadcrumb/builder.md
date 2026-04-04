# Breadcrumb

id: breadcrumb
label: Breadcrumb
family: family-d-navigation
category: Navigation
intent: Show current location in hierarchy
description: Navigation breadcrumb trail
composable: true
capabilities: 
required_parts: 
optional_parts: BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator
tags: breadcrumb, path, navigation, trail, location
keywords: 
pain: Breadcrumbs fail to mark current page correctly for accessibility
promise: Current page state enforced via activity state mapping
why: ActivityState defines whether a breadcrumb link is active. The primitive maps this to aria-current automatically. This ensures correct navigation semantics without manual ARIA handling.
rules: CR-001, CR-004
use_cases: navigation trails, hierarchy display
related: navigation_menu, sidebar, nav_item, pagination, link_group


file: breadcrumb_ui.css
tokens: breadcrumb-*, space-*, font-*, motion-*
foundation: spacing, radius, shadow, typography, motion
states: active
island: breadcrumb_island.rs

pillar: navigation

badges: SSR Safe, Hydration Safe, Token Driven, Deterministic API, Zero Drift

## before
// ❌ Typical
view! {
  <nav>
    <a href="#">Home</a>
    <span>"/"</span>
    <a class="active">Page</a>
  </nav>
}

## after
// ✅ CanonRS
view! {
  <Breadcrumb>
    <BreadcrumbItem>
      <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
    <BreadcrumbItem>
      <BreadcrumbPage>"Page"</BreadcrumbPage>
    </BreadcrumbItem>
  </Breadcrumb>
}
