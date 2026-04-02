use leptos::prelude::*;
use super::tree_ui::{Tree, TreeItem, TreeGroup};

#[component]
pub fn TreeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Tree>
                    <TreeItem has_children=true><span>{"Documents"}</span></TreeItem>
                    <TreeGroup>
                        <TreeItem><span>{"Resume.pdf"}</span></TreeItem>
                        <TreeItem has_children=true><span>{"Projects"}</span></TreeItem>
                        <TreeGroup>
                            <TreeItem><span>{"project-a"}</span></TreeItem>
                            <TreeItem><span>{"project-b"}</span></TreeItem>
                        </TreeGroup>
                    </TreeGroup>
                    <TreeItem has_children=true><span>{"Pictures"}</span></TreeItem>
                    <TreeGroup>
                        <TreeItem><span>{"photo.jpg"}</span></TreeItem>
                        <TreeItem><span>{"avatar.png"}</span></TreeItem>
                    </TreeGroup>
                    <TreeItem><span>{"notes.txt"}</span></TreeItem>
                </Tree>
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
