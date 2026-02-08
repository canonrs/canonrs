//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tree Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn Tree(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-tree="" role="tree" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn TreeItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 0)] depth: usize,
    #[prop(default = false)] selected: bool,
    #[prop(default = false)] expanded: bool,
    #[prop(default = false)] has_children: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-tree-item=""
            attr:data-depth={depth.to_string()}
            attr:data-selected={selected.to_string()}
            attr:data-expanded={expanded.to_string()}
            attr:data-has-children={has_children.to_string()}
            role="treeitem"
            attr:aria-selected={selected.to_string()}
            attr:aria-expanded={if has_children { expanded.to_string() } else { "".to_string() }}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn TreeGroup(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-tree-group="" role="group" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}
