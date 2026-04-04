# Page Header

id: page-header
label: Page Header
family: family-h-layout
category: Display
intent: Page title and actions bar
description: Page header with title and actions
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: page-header, title, heading, actions, breadcrumb
keywords: 
pain: Page headers mix title, actions and breadcrumbs inconsistently
promise: Header structure enforced with explicit semantic regions
why: PageHeaderPrimitive defines structured parts like title, description and actions. Each region is explicitly declared. This guarantees consistent layout composition across pages.
rules: CR-001, CR-004
use_cases: dashboards, admin pages
related: card, resizable, scroll_area, aspect_ratio, toolbar, separator


file: page_header_ui.css
tokens: page-header-*, space-*, font-*, border-*
foundation: spacing, border, typography
states: 
island: page_header_island.rs

## before
// ❌ Typical
view! {
  <div class="header">
    <h1>"Title"</h1>
  </div>
}

## after
// ✅ CanonRS
view! {
  <PageHeader>
    <PageHeaderContent>
      <PageHeaderTitle>"Title"</PageHeaderTitle>
    </PageHeaderContent>
  </PageHeader>
}
