#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::TocItem;
use crate::ui::table_of_contents::table_of_contents_boundary::TableOfContents;
use crate::ui::scroll_area::scroll_area_boundary::ScrollArea;
use canonrs_core::primitives::table_of_contents::TocMode;

#[derive(Clone, Debug, Default)]
pub struct RenderedMarkdown {
    pub html: String,
    pub toc: Vec<TocItem>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TocPosition {
    #[default]
    Top,
    Sidebar,
}

#[component]
pub fn MarkdownSurface(
    rendered: RenderedMarkdown,
    #[prop(default = true)] show_toc: bool,
    #[prop(default = true)] show_toolbar: bool,
    #[prop(default = TocPosition::Top)] toc_position: TocPosition,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    let has_toc = !rendered.toc.is_empty() && show_toc;
    let is_sidebar = toc_position == TocPosition::Sidebar;
    let toc_items = rendered.toc.clone();
    let content_id = format!("{}-content", id);
    let html = rendered.html.clone();
    let _ = show_toolbar;

    view! {
        <div
            data-rs-markdown=""
            data-rs-uid=canonrs_core::infra::uid::generate("md")
            data-rs-interaction="content"
            data-toc-position=if is_sidebar { "sidebar" } else { "top" }
            data-rs-value=id
            style="display:flex;flex-direction:row;gap:var(--space-xl);height:500px"
        >
            {(has_toc && is_sidebar).then(|| view! {
                <aside
                    data-rs-md-toc-sidebar=""
                    style="width:var(--markdown-toc-width);flex-shrink:0;height:100%;overflow-y:auto"
                >
                    <TableOfContents
                        items=toc_items
                        mode=TocMode::Nested
                        title="On this page"
                    />
                </aside>
            })}
            <ScrollArea attr:style="flex:1;min-width:0;max-width:66%;height:100%">
                <div
                    data-rs-markdown-content=""
                    id=content_id
                    inner_html=html
                />
            </ScrollArea>
        </div>
    }
}

#[component]
pub fn MarkdownLayout(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = TocPosition::Sidebar)] toc_position: TocPosition,
) -> impl IntoView {
    let toc_pos = if toc_position == TocPosition::Sidebar { "sidebar" } else { "top" };
    view! {
        <div
            data-rs-markdown=""
            data-rs-component="Markdown"
            data-rs-interaction="content"
            data-toc-position=toc_pos
            data-rs-value=value
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MarkdownContent(
    rendered: RenderedMarkdown,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    let content_id = format!("{}-content", id);
    view! {
        <div
            data-rs-markdown-content=""
            id=content_id
            inner_html=rendered.html
        />
    }
}

#[component]
pub fn MarkdownTOC(
    toc: Vec<TocItem>,
    #[prop(into, default = String::new())] id: String,
    #[prop(into, optional)] scroll_target: Option<String>,
) -> impl IntoView {
    let _ = id;
    let _ = scroll_target;
    view! {
        <aside data-rs-md-toc-sidebar="">
            <TableOfContents
                items=toc
                mode=TocMode::Nested
                title="On this page"
            />
        </aside>
    }
}

#[component]
pub fn MarkdownPreview() -> impl IntoView {
    let rendered = RenderedMarkdown {
        html: "<p>Markdown preview</p>".to_string(),
        toc: vec![],
    };
    view! { <MarkdownSurface rendered=rendered show_toc=false show_toolbar=false /> }
}
