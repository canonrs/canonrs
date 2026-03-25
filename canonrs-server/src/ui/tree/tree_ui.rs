//! @canon-level: ui
//! Tree - attribute-driven
//! Estrutura via TreeItem + TreeGroup aninhados

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
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] has_children: bool,
    #[prop(default = 0)] depth: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TreeItemPrimitive
            selected=selected
            expanded=expanded
            has_children=has_children
            depth=depth
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
            <TreeItem has_children=true expanded=true>"Documents"</TreeItem>
            <TreeGroup>
                <TreeItem depth=1>"Resume.pdf"</TreeItem>
                <TreeItem depth=1 has_children=true>"Projects"</TreeItem>
                <TreeGroup>
                    <TreeItem depth=2>"project-a"</TreeItem>
                    <TreeItem depth=2>"project-b"</TreeItem>
                </TreeGroup>
            </TreeGroup>
            <TreeItem has_children=true>"Pictures"</TreeItem>
            <TreeGroup>
                <TreeItem depth=1>"photo.jpg"</TreeItem>
            </TreeGroup>
        </Tree>
    }
}
