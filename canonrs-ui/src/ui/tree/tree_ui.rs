use leptos::prelude::*;
use crate::primitives::tree::{Tree as TreePrimitive, TreeGroup};
use super::{TreeNode, TreeNodeItem};

#[component]
pub fn Tree(
    nodes: Signal<Vec<TreeNode>>,
    selected_id: Signal<Option<String>>,
    #[prop(default = false)] show_checkboxes: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <TreePrimitive
            attr:data-multiselect={show_checkboxes.to_string()}
            attr:aria-multiselectable={show_checkboxes.to_string()}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            {move || {
                nodes.get().into_iter().enumerate().flat_map(|(idx, node)| {
                    render_node_recursive(
                        node,
                        1,
                        selected_id,
                        show_checkboxes,
                        idx == 0,
                    )
                }).collect_view()
            }}
        </TreePrimitive>
    }
}

fn render_node_recursive(
    node: TreeNode,
    level: usize,
    selected_id: Signal<Option<String>>,
    show_checkboxes: bool,
    is_first: bool,
) -> Vec<AnyView> {
    let node_id = node.id.clone();
    let node_id_for_closure = node_id.clone();
    let children = node.children.clone();
    let is_expanded = node.expanded;
    let has_children = !children.is_empty();
    let is_selected = move || selected_id.get().as_ref() == Some(&node_id_for_closure);

    let tabindex = if is_first { 0 } else { -1 };
    
    // Generate unique group ID for aria-controls
    let group_id = format!("tree-group-{}", node_id);

    let item_view = if has_children {
        view! {
            <TreeNodeItem
                node={node}
                depth={level - 1}
                level={level}
                selected={is_selected()}
                show_checkbox={show_checkboxes}
                tabindex={tabindex}
                group_id={group_id.clone()}
            />
        }
    } else {
        view! {
            <TreeNodeItem
                node={node}
                depth={level - 1}
                level={level}
                selected={is_selected()}
                show_checkbox={show_checkboxes}
                tabindex={tabindex}
            />
        }
    };

    let mut views = vec![item_view.into_any()];

    // Render TreeGroup if has_children
    if has_children {
        views.push(
            view! {
                <TreeGroup id={group_id}>
                    {children.into_iter().flat_map(|child| {
                        render_node_recursive(
                            child,
                            level + 1,
                            selected_id,
                            show_checkboxes,
                            false,
                        )
                    }).collect_view()}
                </TreeGroup>
            }.into_any()
        );
    }

    views
}
