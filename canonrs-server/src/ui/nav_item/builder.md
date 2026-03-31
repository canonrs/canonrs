# Nav Item

id: nav-item
label: Nav Item
family: navigation
category: Navigation
intent: Single navigation link item
description: Single navigation item
composable: false
capabilities: Active, Disabled
required_parts: 
optional_parts: 
tags: nav-item, link, menu-item, navigation, page
keywords: 
pain: Navigation links lack active state and accessibility consistency
promise: Active and disabled navigation states enforced structurally
why: NavItemPrimitive encodes ActivityState and DisabledState into data attributes and ARIA. aria-current is derived automatically. This guarantees consistent navigation behavior and accessibility.
rules: CR-001, CR-004
use_cases: sidebars, menus
related: navigation_menu, sidebar, breadcrumb, pagination, link_group

## before
// ❌ Typical
view! {
  <a class="active" href="/">"Home"</a>
}

## after
// ✅ CanonRS
view! {
  <NavItem label="Home" active=true />
}
