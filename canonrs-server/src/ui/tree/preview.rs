use leptos::prelude::*;
use super::tree_island::TreeIsland;
use super::tree_node::TreeNode;
use super::tree_ui::{Tree, TreeItem, TreeGroup};

#[component]
pub fn TreeShowcasePreview() -> impl IntoView {
    let nodes = vec![
        TreeNode::new("documents", "Documents", "folder")
            .with_expanded(true)
            .with_children(vec![
                TreeNode::new("resume", "Resume.pdf", "file"),
                TreeNode::new("projects", "Projects", "folder")
                    .with_expanded(true)
                    .with_children(vec![
                        TreeNode::new("project-a", "project-a", "file"),
                        TreeNode::new("project-b", "project-b", "file"),
                    ]),
            ]),
        TreeNode::new("pictures", "Pictures", "folder")
            .with_children(vec![
                TreeNode::new("photo",  "photo.jpg",  "file"),
                TreeNode::new("avatar", "avatar.png", "file"),
            ]),
        TreeNode::new("notes", "notes.txt", "file"),
    ];

    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TreeIsland nodes=nodes />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Hierarchy state fully governed via structured attributes."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Deep nesting"</span>
                <div data-rs-showcase-preview-row="">
                    <Tree>
                        <TreeItem has_children=true><span>{"src"}</span></TreeItem>
                        <TreeGroup>
                            <TreeItem has_children=true><span>{"components"}</span></TreeItem>
                            <TreeGroup>
                                <TreeItem has_children=true><span>{"ui"}</span></TreeItem>
                                <TreeGroup>
                                    <TreeItem><span>{"button.rs"}</span></TreeItem>
                                    <TreeItem><span>{"input.rs"}</span></TreeItem>
                                </TreeGroup>
                            </TreeGroup>
                            <TreeItem><span>{"main.rs"}</span></TreeItem>
                            <TreeItem><span>{"lib.rs"}</span></TreeItem>
                        </TreeGroup>
                    </Tree>
                </div>
            </div>
        </div>
    }
}
