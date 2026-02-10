use leptos::prelude::*;
use crate::primitives::tree::TreeItem as TreeItemPrimitive;
use crate::ui::checkbox::Checkbox;
use super::tree_node::TreeNode;

#[component]
pub fn TreeNodeItem(
    node: TreeNode,
    depth: usize,
    selected: bool,
    #[prop(default = false)] show_checkbox: bool,
) -> impl IntoView {
    let has_children = node.has_children();
    let is_expanded = node.expanded;
    let is_checked = node.checked;

    view! {
        <TreeItemPrimitive
            depth={depth}
            selected={selected}
            expanded={is_expanded}
            has_children={has_children}
        >
            <div attr:data-tree-toggle="">
                {if has_children {
                    if is_expanded { "▼" } else { "▶" }
                } else {
                    ""
                }}
            </div>

            {show_checkbox.then(|| {
                view! {
                    <div attr:data-tree-checkbox-wrapper="">
                        <Checkbox id=format!("tree-checkbox-{}", node.id) checked=is_checked />
                    </div>
                }
            })}

            {node.icon.map(|icon| view! {
                <span attr:data-tree-icon="">{icon}</span>
            })}

            <span attr:data-tree-label="">{node.label}</span>
        </TreeItemPrimitive>
    }
}
