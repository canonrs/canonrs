use leptos::prelude::*;
use leptos::callback::Callback;
use canonrs_ui::primitives::tree::Tree as TreePrimitive;
use super::{TreeNode, TreeNodeItem};

#[component]
pub fn Tree(
    nodes: Signal<Vec<TreeNode>>,
    selected_id: Signal<Option<String>>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
    #[prop(default = false)] show_checkboxes: bool,
    #[prop(optional)] on_check: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let on_check_cb = on_check.unwrap_or_else(|| Callback::new(|_| {}));

    view! {
        <TreePrimitive
            attr:data-multiselect={show_checkboxes.to_string()}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            {move || {
                nodes.get().into_iter().map(|node| {
                    render_node_recursive(
                        node,
                        0,
                        selected_id,
                        on_select,
                        on_toggle,
                        show_checkboxes,
                        on_check_cb,
                    )
                }).collect_view()
            }}
        </TreePrimitive>
    }
}

fn render_node_recursive(
    node: TreeNode,
    depth: usize,
    selected_id: Signal<Option<String>>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
    show_checkboxes: bool,
    on_check: Callback<String>,
) -> AnyView {
    let node_id = node.id.clone();
    let children = node.children.clone();
    let is_expanded = node.expanded;
    let is_selected = move || selected_id.get().as_ref() == Some(&node_id);

    view! {
        <div data-tree-node-wrapper="">
            <TreeNodeItem
                node={node}
                depth={depth}
                selected={is_selected()}
                show_checkbox={show_checkboxes}
                on_check={on_check}
                on_select={on_select}
                on_toggle={on_toggle}
            />

            {(is_expanded && !children.is_empty()).then(|| {
                children.into_iter().map(|child| {
                    render_node_recursive(
                        child,
                        depth + 1,
                        selected_id,
                        on_select,
                        on_toggle,
                        show_checkboxes,
                        on_check,
                    )
                }).collect_view()
            })}
        </div>
    }.into_any()
}
