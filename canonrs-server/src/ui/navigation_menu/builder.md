# Navigation Menu

id: navigation-menu
label: Navigation Menu
family: navigation
category: Navigation
intent: Primary site navigation with submenus
description: Navigation menu
composable: true
capabilities: OpenClose
required_parts: NavigationMenuList, NavigationMenuItem
optional_parts: NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink
tags: navigation-menu, navigation, menu, nav, links, site, header
keywords: 
pain: Nested navigation menus require id wiring and break interaction consistency
promise: Trigger-content relationship enforced without id wiring
why: NavigationMenu uses DOM structure and data-rs-state instead of ids. Trigger and content are linked via closest/sibling logic. This guarantees stable interaction without manual wiring.
rules: CR-001, CR-004
use_cases: site navigation, dropdown navigation
related: sidebar, nav_item, breadcrumb, pagination, link_group

## before
// ❌ Typical
view! {
  <button id="trigger">"Menu"</button>
  <div id="content">"Items"</div>
}

## after
// ✅ CanonRS
view! {
  <NavigationMenu>
    <NavigationMenuItem>
      <NavigationMenuTrigger>"Menu"</NavigationMenuTrigger>
      <NavigationMenuContent>"Items"</NavigationMenuContent>
    </NavigationMenuItem>
  </NavigationMenu>
}
