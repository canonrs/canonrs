use leptos::prelude::*;
use crate::primitives::tree::TreeItem as TreeItemPrimitive;
use crate::ui::checkbox::Checkbox;
use super::tree_node::TreeNode;

#[component]
pub fn TreeNodeItem(
    node: TreeNode,
    depth: usize,
    level: usize,
    selected: bool,
    tabindex: i32,
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
            tabindex={tabindex}
        >
            {has_children.then(|| {
                view! {
                    <div data-tree-toggle="">
                        {if is_expanded { "▼" } else { "▶" }}
                    </div>
                }
            })}

            {show_checkbox.then(|| {
                view! {
                    <div data-tree-checkbox-wrapper="">
                        <Checkbox id=format!("tree-checkbox-{}", node.id) checked=is_checked />
                    </div>
                }
            })}

            {node.icon.map(|icon| view! {
                <span data-tree-icon="">{icon}</span>
            })}

            <span data-tree-label="" attr:aria-level={level.to_string()}>
                {node.label.clone()}
            </span>
        </TreeItemPrimitive>
    }
}
