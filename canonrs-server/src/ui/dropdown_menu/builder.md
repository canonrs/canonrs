# Dropdown Menu

id: dropdown-menu
label: Dropdown Menu
family: interactive
category: Action
intent: Show contextual action menu
description: Dropdown menu
composable: true
capabilities: OpenClose, Disabled
required_parts: DropdownMenuTrigger, DropdownMenuContent
optional_parts: DropdownMenuItem, DropdownMenuSeparator, DropdownMenuGroup
tags: dropdown-menu, dropdown, menu, options, actions
keywords: 
pain: Dropdown menus break keyboard navigation and selection state
promise: Menu interaction and state fully encoded via primitives
why: DropdownMenuPrimitive defines trigger/content with ARIA roles. ToggleState and ActivityState manage selection and highlight. This guarantees consistent menu behavior.
rules: CR-001, CR-004
use_cases: actions menu, user menu
related: context_menu, menubar, command

## before
// ❌ Typical
view! {
  <div class="dropdown">
    <button>"Open"</button>
    <div class="menu">"Item"</div>
  </div>
}

## after
// ✅ CanonRS
view! {
  <DropdownMenu>
    <DropdownMenuTrigger>"Open"</DropdownMenuTrigger>
    <DropdownMenuContent>
      <DropdownMenuItem>"Item"</DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
}
