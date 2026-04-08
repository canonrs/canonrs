//! @canon-level: strict
//! Tree Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::tree_ui::{Tree, TreeItem, TreeGroup};
use super::tree_node::TreeNode;

#[island]
pub fn TreeInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
                use wasm_bindgen_futures::spawn_local;
        spawn_local(async move {
            canonrs_client::interactions::tree::init_all();
        });
    }
    view! { <></> }
}

fn render_nodes(nodes: Vec<TreeNode>) -> impl IntoView {
    nodes.into_iter().map(|node| {
        let has_children = !node.children.is_empty();
        let children     = node.children.clone();
        let label        = node.label.clone();
        view! {
            <TreeItem has_children=has_children>{label}</TreeItem>
            {if has_children {
                view! { <TreeGroup>{render_nodes(children)}</TreeGroup> }.into_any()
            } else {
                view! {}.into_any()
            }}
        }
    }).collect::<Vec<_>>()
}

#[component]
pub fn TreeIsland(
    nodes: Vec<TreeNode>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    view! {
        <TreeInit />
        <Tree class=class>
            {render_nodes(nodes)}
        </Tree>
    }
}

#[component]
pub fn TreeItemIsland(
    children: Children,
    #[prop(default = false)] has_children: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TreeItem has_children=has_children class=class>{children()}</TreeItem> }
}

#[component]
pub fn TreeGroupIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TreeGroup class=class>{children()}</TreeGroup> }
}
