use leptos::prelude::*;
use super::tree_boundary::{Tree, TreeItem, TreeGroup};

#[component]
pub fn TreeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Tree>
                    <TreeItem depth=0>"Documents"</TreeItem>
                    <TreeItem has_children=true depth=0>"Projects"</TreeItem>
                    <TreeGroup>
                        <TreeItem depth=1>"canonrs"</TreeItem>
                        <TreeItem depth=1>"monorepo"</TreeItem>
                    </TreeGroup>
                    <TreeItem depth=0>"Settings"</TreeItem>
                </Tree>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tree structure governed by DOM — keyboard nav and expand/collapse via data-rs-expanded."
            </p>
        </div>
    }
}
