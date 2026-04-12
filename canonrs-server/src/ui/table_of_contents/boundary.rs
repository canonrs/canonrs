//! @canon-level: strict
//! TableOfContents Boundary — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::table_of_contents_ui::TableOfContents as TableOfContentsUi;
use canonrs_core::TocItem;
use canonrs_core::primitives::table_of_contents::TocMode;

#[component]
pub fn TableOfContents(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = vec![])] items: Vec<TocItem>,
    #[prop(into, default = String::from("On this page"))] title: String,
    #[prop(optional)] mode: Option<TocMode>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let toc_mode = mode.unwrap_or(TocMode::Simple);
    let _ = children; // children ignored — items-based rendering
    view! { <TableOfContentsUi items=items title=title mode=toc_mode class=class /> }
}
