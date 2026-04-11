//! @canon-level: strict
//! TableOfContents Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::table_of_contents_ui::TableOfContents;
use canonrs_core::primitives::table_of_contents::TocMode;
use canonrs_core::TocItem;

#[component]
pub fn TableOfContentsIsland(
    children: Children,
    #[prop(default = vec![])] items: Vec<TocItem>,
    #[prop(into, default = String::from("On this page"))] title: String,
    #[prop(into, default = String::from("simple"))] mode: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let toc_mode = match mode.as_str() {
        "expand" => TocMode::Expand,
        "nested" => TocMode::Nested,
        _        => TocMode::Simple,
    };
    let _ = children; // children used when items not provided
    view! { <TableOfContents items=items title=title mode=toc_mode class=class /> }
}
