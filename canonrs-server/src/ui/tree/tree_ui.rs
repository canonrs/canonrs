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
    #[prop(default = 0u8)] depth: u8,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    if has_children {
        view! {
            <TreeItemPrimitive has_children=true depth=depth class=class>
                <span data-rs-tree-toggle="">
                    <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="9 18 15 12 9 6"/>
                    </svg>
                </span>
                {children()}
            </TreeItemPrimitive>
        }.into_any()
    } else {
        view! {
            <TreeItemPrimitive has_children=false depth=depth class=class>
                <span data-rs-tree-indent="" />
                {children()}
            </TreeItemPrimitive>
        }.into_any()
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
            <TreeItem depth=0>"Documents"</TreeItem>
            <TreeItem has_children=true depth=0>"Projects"</TreeItem>
            <TreeGroup>
                <TreeItem depth=1>"canonrs"</TreeItem>
                <TreeItem depth=1>"monorepo"</TreeItem>
            </TreeGroup>
            <TreeItem depth=0>"Settings"</TreeItem>
        </Tree>
    }
}
