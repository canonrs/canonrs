use leptos::prelude::*;
use super::tree_ui::{Tree, TreeItem, TreeGroup};
use super::tree_node::TreeNode;

fn render_nodes(nodes: Vec<TreeNode>) -> impl IntoView {
    nodes.into_iter().map(|node| {
        let has_children = !node.children.is_empty();
        let children     = node.children.clone();
        let label        = node.label.clone();

        view! {
            <TreeItem has_children=has_children>
                {label}
            </TreeItem>
            {if has_children {
                view! {
                    <TreeGroup>
                        {render_nodes(children)}
                    </TreeGroup>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        }
    }).collect::<Vec<_>>()
}

#[island]
pub fn TreeIsland(
    nodes: Vec<TreeNode>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! {
        <Tree class=class>
            {render_nodes(nodes)}
        </Tree>
    }
}
