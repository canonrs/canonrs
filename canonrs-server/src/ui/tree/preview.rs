use leptos::prelude::*;
use super::tree_island::{TreeIsland, TreeItemIsland, TreeGroupIsland};

#[component]
pub fn TreeShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TreeIsland>
                    <TreeItemIsland depth=0>"Documents"</TreeItemIsland>
                    <TreeItemIsland has_children=true depth=0>"Projects"</TreeItemIsland>
                    <TreeGroupIsland>
                        <TreeItemIsland depth=1>"canonrs"</TreeItemIsland>
                        <TreeItemIsland depth=1>"monorepo"</TreeItemIsland>
                    </TreeGroupIsland>
                    <TreeItemIsland depth=0>"Settings"</TreeItemIsland>
                </TreeIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tree structure governed by DOM — keyboard nav and expand/collapse via data-rs-expanded."
            </p>
        </div>
    }
}
