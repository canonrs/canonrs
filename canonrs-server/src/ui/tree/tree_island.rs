//! Tree Island — Canon Rule passthrough
use leptos::prelude::*;
use super::tree_ui::{Tree, TreeItem, TreeGroup};

#[component]
pub fn TreeIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <Tree class=class>{children()}</Tree> }
}

#[component]
pub fn TreeItemIsland(
    children: Children,
    #[prop(default = false)] has_children: bool,
    #[prop(default = 0u8)] depth: u8,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TreeItem has_children=has_children depth=depth class=class>{children()}</TreeItem> }
}

#[component]
pub fn TreeGroupIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TreeGroup class=class>{children()}</TreeGroup> }
}
