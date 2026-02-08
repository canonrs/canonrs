use leptos::prelude::*;
use super::{Tree, TreeNode};

pub fn basic_example() -> impl IntoView {
    let nodes = vec![
        TreeNode {
            id: "1".to_string(),
            label: "Root Node 1".to_string(),
            node_type: "folder".to_string(),
            icon: Some("📁".to_string()),
            expanded: false,
            checked: false,
            children: vec![
                TreeNode {
                    id: "1-1".to_string(),
                    label: "Child 1.1".to_string(),
                    node_type: "file".to_string(),
                    icon: Some("📄".to_string()),
                    expanded: false,
                    checked: false,
                    children: vec![],
                    metadata: None,
                },
                TreeNode {
                    id: "1-2".to_string(),
                    label: "Child 1.2".to_string(),
                    node_type: "file".to_string(),
                    icon: Some("📄".to_string()),
                    expanded: false,
                    checked: false,
                    children: vec![],
                    metadata: None,
                },
            ],
            metadata: None,
        },
        TreeNode {
            id: "2".to_string(),
            label: "Root Node 2".to_string(),
            node_type: "folder".to_string(),
            icon: Some("📁".to_string()),
            expanded: false,
            checked: false,
            children: vec![
                TreeNode {
                    id: "2-1".to_string(),
                    label: "Child 2.1".to_string(),
                    node_type: "file".to_string(),
                    icon: Some("📄".to_string()),
                    expanded: false,
                    checked: false,
                    children: vec![],
                    metadata: None,
                },
            ],
            metadata: None,
        },
    ];

    view! {
        <TreeExample initial_nodes=nodes />
    }
}

#[component]
fn TreeExample(initial_nodes: Vec<TreeNode>) -> impl IntoView {
    let nodes = create_rw_signal(initial_nodes);
    let selected_id = create_rw_signal(None::<String>);

    let on_select = Callback::new(move |id: String| {
        selected_id.set(Some(id));
    });

    let on_toggle = Callback::new(move |id: String| {
        nodes.update(|nodes| toggle_node(nodes, &id));
    });

    view! {
        <Tree
            nodes=nodes.into()
            selected_id=selected_id.into()
            on_select=on_select
            on_toggle=on_toggle
        />
    }
}

fn toggle_node(nodes: &mut Vec<TreeNode>, id: &str) {
    for node in nodes.iter_mut() {
        if node.id == id {
            node.expanded = !node.expanded;
            return;
        }
        toggle_node(&mut node.children, id);
    }
}
