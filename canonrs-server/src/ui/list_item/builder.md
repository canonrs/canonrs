# List Item

id: list-item
label: List Item
family: family-f-data
category: Display
intent: Display a single item in a list
description: Single list item with title and description
composable: true
capabilities: Selected, Disabled
required_parts: 
optional_parts: ListItemTitle, ListItemDescription
tags: list-item, item, row, entry, element
keywords: 
pain: Lists lack consistent selection, disabled and accessibility states
promise: Selection and interaction states encoded via structured attributes
why: ListItem encodes selectable, selected and disabled states into data attributes. ARIA attributes are derived automatically. This guarantees consistent list interaction behavior.
rules: CR-001, CR-004
use_cases: menus, lists
related: table, data_table, virtual_list, empty_table, tree


file: list_item_ui.css
tokens: list-item-*, list-*, space-*, radius-*, font-*
foundation: spacing, radius, motion, typography
states: selected
island: list_item_island.rs

## before
// ❌ Typical
view! {
  <li class="active">"Item"</li>
}

## after
// ✅ CanonRS
view! {
  <List>
    <ListItem selectable=true selected=true>"Item"</ListItem>
  </List>
}
