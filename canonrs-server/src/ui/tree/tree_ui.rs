#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    TreePrimitive, TreeItemPrimitive, TreeGroupPrimitive,
    };

#[component]
pub fn Tree(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <TreePrimitive class=class>{children()}</TreePrimitive> }
}
#[component]
pub fn TreeItem(children: Children, #[prop(default = false)] has_children: bool, #[prop(default = 0u8)] depth: u8, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! {
        <TreeItemPrimitive has_children=has_children depth=depth class=class>
            {if has_children {
                view! { <span data-rs-tree-toggle=""/> }.into_any()
            } else {
                view! { <span data-rs-tree-indent=""/> }.into_any()
            }}
            <span data-rs-tree-icon=""/>
            {children()}
        </TreeItemPrimitive>
    }
}
#[component]
pub fn TreeGroup(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <TreeGroupPrimitive class=class>{children()}</TreeGroupPrimitive> }
}
