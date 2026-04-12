//! Tree Island — Canon Rule passthrough
use leptos::prelude::*;
use super::tree_ui::{
    Tree as TreeUi,
    TreeItem as TreeItemUi,
    TreeGroup as TreeGroupUi
};

#[component]
pub fn Tree(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TreeUi class=class>{children()}</TreeUi> }
}

#[component]
pub fn TreeItem(
    children: Children,
    #[prop(default = false)] has_children: bool,
    #[prop(default = 0u8)] depth: u8,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TreeItemUi has_children=has_children depth=depth class=class>{children()}</TreeItemUi> }
}

#[component]
pub fn TreeGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <TreeGroupUi class=class>{children()}</TreeGroupUi> }
}
