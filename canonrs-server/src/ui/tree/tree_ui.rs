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
                    <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="9 18 15 12 9 6"/>
                    </svg>
                </span>
                <span data-rs-tree-icon="" data-rs-tree-icon-folder="">
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                    </svg>
                </span>
                {children()}
            </TreeItemPrimitive>
        }.into_any()
    } else {
        view! {
            <TreeItemPrimitive has_children=false depth=depth class=class>
                <span data-rs-tree-indent="" />
                <span data-rs-tree-icon="" data-rs-tree-icon-file="">
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
                        <polyline points="13 2 13 9 20 9"/>
                    </svg>
                </span>
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
