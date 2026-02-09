use leptos::prelude::*;
use super::{Tree, TreeNode};

pub fn basic_example() -> impl IntoView {
    let nodes = RwSignal::new(vec![
        TreeNode {
            id: "1".to_string(),
            label: "Documents".to_string(),
            node_type: "folder".to_string(),
            icon: None,
            expanded: true,
            checked: false,
            metadata: None,
            children: vec![
                TreeNode {
                    id: "1-1".to_string(),
                    label: "Work".to_string(),
                    node_type: "folder".to_string(),
                    icon: None,
                    expanded: false,
                    checked: false,
                    metadata: None,
                    children: vec![],
                },
                TreeNode {
                    id: "1-2".to_string(),
                    label: "Personal".to_string(),
                    node_type: "folder".to_string(),
                    icon: None,
                    expanded: false,
                    checked: false,
                    metadata: None,
                    children: vec![],
                },
            ],
        },
        TreeNode {
            id: "2".to_string(),
            label: "Images".to_string(),
            node_type: "folder".to_string(),
            icon: None,
            expanded: false,
            checked: false,
            metadata: None,
            children: vec![],
        },
    ]);

    let selected = RwSignal::new(None);

    view! {
        <Tree nodes=nodes.into() selected_id=selected.into() />
    }
}

#[component]
pub fn BasicExample() -> impl IntoView {
    basic_example()
}

pub fn with_checkboxes_example() -> impl IntoView {
    let nodes = RwSignal::new(vec![
        TreeNode {
            id: "1".to_string(),
            label: "Select All".to_string(),
            node_type: "group".to_string(),
            icon: None,
            expanded: true,
            checked: false,
            metadata: None,
            children: vec![
                TreeNode {
                    id: "1-1".to_string(),
                    label: "Option 1".to_string(),
                    node_type: "item".to_string(),
                    icon: None,
                    expanded: false,
                    checked: false,
                    metadata: None,
                    children: vec![],
                },
                TreeNode {
                    id: "1-2".to_string(),
                    label: "Option 2".to_string(),
                    node_type: "item".to_string(),
                    icon: None,
                    expanded: false,
                    checked: true,
                    metadata: None,
                    children: vec![],
                },
            ],
        },
    ]);

    let selected = RwSignal::new(None);

    view! {
        <Tree nodes=nodes.into() selected_id=selected.into() show_checkboxes=true />
    }
}

pub fn with_icons_example() -> impl IntoView {
    let nodes = RwSignal::new(vec![
        TreeNode {
            id: "1".to_string(),
            label: "Folder".to_string(),
            node_type: "folder".to_string(),
            icon: Some("📁".to_string()),
            expanded: true,
            checked: false,
            metadata: None,
            children: vec![
                TreeNode {
                    id: "1-1".to_string(),
                    label: "File.txt".to_string(),
                    node_type: "file".to_string(),
                    icon: Some("📄".to_string()),
                    expanded: false,
                    checked: false,
                    metadata: None,
                    children: vec![],
                },
            ],
        },
    ]);

    let selected = RwSignal::new(None);

    view! {
        <Tree nodes=nodes.into() selected_id=selected.into() />
    }
}
