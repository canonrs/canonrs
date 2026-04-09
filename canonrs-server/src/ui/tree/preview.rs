use leptos::prelude::*;
use super::tree_island::{TreeIsland, TreeItemIsland, TreeGroupIsland};

#[component]
pub fn TreeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TreeIsland>
                    <TreeItemIsland>"Documents"</TreeItemIsland>
                    <TreeItemIsland has_children=true>
                        "Projects"
                        <TreeGroupIsland>
                            <TreeItemIsland>"canonrs"</TreeItemIsland>
                            <TreeItemIsland>"monorepo"</TreeItemIsland>
                        </TreeGroupIsland>
                    </TreeItemIsland>
                    <TreeItemIsland>"Settings"</TreeItemIsland>
                </TreeIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tree structure governed by DOM — keyboard nav and expand/collapse via data-rs-state."
            </p>
        </div>
    }
}
