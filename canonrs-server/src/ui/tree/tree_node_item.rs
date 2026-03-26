use leptos::prelude::*;
use canonrs_core::primitives::{TreeItemPrimitive};

#[component]
pub fn TreeNodeItem(
    children: Children,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] has_children: bool,
    #[prop(default = 0)] depth: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TreeItemPrimitive
            selected=selected.into()
            expanded=expanded.into()
            has_children=has_children
            depth=depth
            class=class
        >
            {children()}
        </TreeItemPrimitive>
    }
}
