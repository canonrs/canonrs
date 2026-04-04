# Tree

id: tree
label: Tree
family: family-b-selection
category: Display
intent: Display hierarchical data
description: Tree view component
composable: true
capabilities: Selected
required_parts: TreeItem
optional_parts: TreeGroup
tags: tree, hierarchy, nodes, structure, explorer
keywords: 
pain: Tree structures lose selection, expansion and focus consistency
promise: Hierarchy state fully governed via structured attributes
why: TreePrimitive and TreeItemPrimitive encode selection, focus and expansion via data attributes. ARIA roles are enforced. This guarantees accessible hierarchical navigation.
rules: CR-001, CR-004
use_cases: file explorers, nested navigation
related: table, data_table, virtual_list, empty_table, list_item


file: tree_ui.css
tokens: tree-*, space-*, size-*, radius-*, font-*, motion-*
foundation: spacing, size, radius, motion, typography
states: open, closed, selected, disabled, active
island: tree_island.rs

## before
// ❌ Typical
view! {
  <ul>
    <li>"Item"</li>
  </ul>
}

## after
// ✅ CanonRS
view! {
  <Tree>
    <TreeItem has_children=true>"Item"</TreeItem>
  </Tree>
}
