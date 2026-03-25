//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tree Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn TreePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-tree="" role="tree" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn TreeItemPrimitive(
    children: Children,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] has_children: bool,
    #[prop(default = 0)] depth: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tree-item=""
            data-rs-state={if selected { "selected" } else { "unselected" }}
            data-rs-expanded={if has_children { Some(if expanded { "true" } else { "false" }) } else { None }}
            data-rs-depth=depth.to_string()
            role="treeitem"
            tabindex=if selected { 0 } else { -1 }
            aria-selected=selected.to_string()
            aria-expanded={if has_children { Some(if expanded { "true" } else { "false" }) } else { None }}
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TreeGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-tree-group="" role="group" class=class>
            {children()}
        </div>
    }
}
