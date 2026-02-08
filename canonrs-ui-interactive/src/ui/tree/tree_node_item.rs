use leptos::prelude::*;
use leptos::callback::Callback;
use leptos::either::Either;
use canonrs_ui::primitives::tree::TreeItem as TreeItemPrimitive;
use canonrs_ui::ui::checkbox::Checkbox;
use super::tree_node::TreeNode;

#[component]
pub fn TreeNodeItem(
    node: TreeNode,
    depth: usize,
    selected: bool,
    #[prop(default = false)] show_checkbox: bool,
    #[prop(optional)] on_check: Option<Callback<String>>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
) -> impl IntoView {
    let node_id = node.id.clone();
    let node_id_toggle = node.id.clone();
    let node_id_check = StoredValue::new(node.id.clone());
    let on_check_stored = StoredValue::new(on_check.clone());
    let has_children = node.has_children();
    let is_expanded = node.expanded;
    let is_checked = node.checked;
    let icon_opt = StoredValue::new(node.icon.clone());
    let label = node.label.clone();

    view! {
        <TreeItemPrimitive
            depth={depth}
            selected={selected}
            expanded={is_expanded}
            has_children={has_children}
            on:click=move |_| on_select.run(node_id.clone())
        >
            <div
                attr:data-tree-toggle=""
                on:click=move |ev| {
                    ev.stop_propagation();
                    on_toggle.run(node_id_toggle.clone());
                }
            >
                {if has_children {
                    if is_expanded { "▼" } else { "▶" }
                } else {
                    ""
                }}
            </div>

            {move || {
                if show_checkbox {
                    Either::Left(view! {
                        <div
                            attr:data-tree-checkbox-wrapper=""
                            on:click=move |ev| {
                                ev.stop_propagation();
                                if let Some(ref handler) = on_check_stored.get_value() {
                                    handler.run(node_id_check.get_value());
                                }
                            }
                        >
                            <Checkbox checked=is_checked />
                        </div>
                    })
                } else {
                    Either::Right(())
                }
            }}

            {move || {
                if let Some(icon) = icon_opt.get_value() {
                    Either::Left(view! { <span attr:data-tree-icon="">{icon}</span> })
                } else {
                    Either::Right(())
                }
            }}

            <span attr:data-tree-label="">{label}</span>
        </TreeItemPrimitive>
    }
}
