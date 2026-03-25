use leptos::prelude::*;
use super::{Tree, TreeItem, TreeGroup};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Tree>
            <TreeItem has_children=true expanded=true>"📁 Documents"</TreeItem>
            <TreeGroup>
                <TreeItem depth=1 has_children=true>"📁 Work"</TreeItem>
                <TreeGroup>
                    <TreeItem depth=2>"📄 report.pdf"</TreeItem>
                </TreeGroup>
                <TreeItem depth=1>"📁 Personal"</TreeItem>
            </TreeGroup>
            <TreeItem has_children=true>"📁 Images"</TreeItem>
            <TreeGroup>
                <TreeItem depth=1>"🖼 Vacation.jpg"</TreeItem>
                <TreeItem depth=1>"🖼 Profile.png"</TreeItem>
            </TreeGroup>
        </Tree>
    }
}

#[component]
pub fn WithIconsExample() -> impl IntoView {
    view! {
        <Tree>
            <TreeItem has_children=true expanded=true>"📁 Folder"</TreeItem>
            <TreeGroup>
                <TreeItem depth=1>"📄 File.txt"</TreeItem>
            </TreeGroup>
        </Tree>
    }
}
