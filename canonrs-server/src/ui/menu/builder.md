# Menu

id: menu
label: Menu
family: utility
category: Navigation
intent: Static vertical menu list
description: Menu component
composable: false
capabilities: 
required_parts: 
optional_parts: 
tags: menu, list, options, actions, items
keywords: 
pain: Menus lack consistent selection, disabled and focus behavior
promise: Menu interaction fully governed via structured ARIA and state attributes
why: MenuItemPrimitive encodes selection, disabled and activity states into data-rs and ARIA attributes. Navigation semantics are enforced at the container level. This guarantees predictable keyboard and accessibility behavior.
rules: CR-001, CR-004
use_cases: dropdown lists, action menus
related: dropdown_menu, context_menu, menubar, command

## before
// ❌ Typical
view! {
  <div class="menu">
    <button class="active">"Item"</button>
  </div>
}

## after
// ✅ CanonRS
view! {
  <Menu>
    <MenuItem selected=true>"Item"</MenuItem>
  </Menu>
}
