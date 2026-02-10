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
    #[prop(default = 0)] tabindex: i32,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let aria_expanded = if has_children {
        Some(expanded.to_string())
    } else {
        None
    };
    
    view! {
        <div
            data-tree-item=""
            data-depth={depth.to_string()}
            data-selected={selected.to_string()}
            data-expanded={expanded.to_string()}
            data-has-children={has_children.to_string()}
            role="treeitem"
            tabindex={tabindex}
            attr:aria-selected={selected.to_string()}
            attr:aria-expanded={aria_expanded}
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
