//! @canon-id: tree
//! @canon-label: Tree
//! @canon-family: data_display
//! @canon-category: Display
//! @canon-intent: Display hierarchical data
//! @canon-description: Tree view component
//! @canon-composable: true
//! @canon-capabilities: Selected
//! @canon-required-parts: TreeItem
//! @canon-optional-parts: TreeGroup
//! @canon-tags: tree, hierarchy, nodes, structure, explorer

use leptos::prelude::*;
use canonrs_core::primitives::{
    TreePrimitive,
    TreeItemPrimitive,
    TreeGroupPrimitive,
};

#[component]
pub fn Tree(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TreePrimitive class=class>
            {children()}
        </TreePrimitive>
    }
}

#[component]
pub fn TreeItem(
    children: Children,
    #[prop(default = false)] has_children: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TreeItemPrimitive
            has_children=has_children
            class=class
        >
            {children()}
        </TreeItemPrimitive>
    }
}

#[component]
pub fn TreeGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TreeGroupPrimitive class=class>
            {children()}
        </TreeGroupPrimitive>
    }
}

#[component]
pub fn TreePreview() -> impl IntoView {
    view! {
        <Tree>
            <TreeItem has_children=true>"Documents"</TreeItem>
            <TreeGroup>
                <TreeItem>"Resume.pdf"</TreeItem>
                <TreeItem has_children=true>"Projects"</TreeItem>
                <TreeGroup>
                    <TreeItem>"project-a"</TreeItem>
                    <TreeItem>"project-b"</TreeItem>
                </TreeGroup>
            </TreeGroup>
            <TreeItem has_children=true>"Pictures"</TreeItem>
            <TreeGroup>
                <TreeItem>"photo.jpg"</TreeItem>
            </TreeGroup>
        </Tree>
    }
}
