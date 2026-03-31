# Sidebar

id: sidebar
label: Sidebar
family: navigation
category: Navigation
intent: Vertical navigation panel
description: Sidebar navigation component
composable: true
capabilities: OpenClose
required_parts: SidebarContent
optional_parts: SidebarHeader, SidebarFooter, SidebarMenu, SidebarMenuItem
tags: sidebar, nav, navigation, links, left-panel
keywords: 
pain: Sidebars lose active state, disabled links and structural consistency
promise: Navigation state and structure enforced at component level
why: SidebarPrimitive encodes visibility, variant and navigation semantics. Menu items derive active and disabled states automatically. This guarantees consistent navigation behavior.
rules: CR-001, CR-004
use_cases: app navigation, admin panels
related: navigation_menu, nav_item, breadcrumb, pagination, link_group

## before
// ❌ Typical
view! {
  <aside>
    <a class="active">"Home"</a>
  </aside>
}

## after
// ✅ CanonRS
view! {
  <Sidebar>
    <SidebarMenu>
      <SidebarMenuItem active=ActivityState::Active href="/">"Home"</SidebarMenuItem>
    </SidebarMenu>
  </Sidebar>
}
