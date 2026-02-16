use leptos::prelude::*;
use canonrs_shared::TocItem;
use crate::primitives::markdown::*;
use crate::ui::table_of_contents::{TableOfContents, TocMode};

#[derive(Clone, Debug)]
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
    let rendered = StoredValue::new(rendered);

    match toc_position {
        TocPosition::Top => view! {
            <MarkdownPrimitive id=id.clone() attr:data-toc-position="top">
                {show_toolbar.then(|| view! {
                    <MarkdownToolbarPrimitive>
                        {has_toc.then(|| view! {
                            <MarkdownToolbarItemPrimitive action="toggle-toc">
                                "Contents"
                            </MarkdownToolbarItemPrimitive>
                        })}
                    </MarkdownToolbarPrimitive>
                })}
                <div data-md-layout="">
                    {has_toc.then(|| {
                        let items = rendered.get_value().toc.clone();
                        view! {
                            <MarkdownTocPrimitive state="closed">
                                <TableOfContents
                                    items=items
                                    mode=TocMode::Simple
                                    id=format!("{}-toc", id)
                                />
                            </MarkdownTocPrimitive>
                        }
                    })}
                    <MarkdownContentPrimitive>
                        <div
                            data-md-content=""
                            inner_html={move || rendered.get_value().html}
                        ></div>
                    </MarkdownContentPrimitive>
                </div>
            </MarkdownPrimitive>
        }.into_any(),

        TocPosition::Sidebar => view! {
            <MarkdownPrimitive id=id.clone() attr:data-toc-position="sidebar">
                <div data-md-layout="">
                    {has_toc.then(|| {
                        let items = rendered.get_value().toc.clone();
                        view! {
                            <aside data-md-toc-sidebar="">
                                <TableOfContents
                                    items=items
                                    mode=TocMode::Simple
                                    id=format!("{}-toc", id)
                                    title="On this page"
                                />
                            </aside>
                        }
                    })}
                    <MarkdownContentPrimitive>
                        <div
                            data-md-content=""
                            inner_html={move || rendered.get_value().html}
                        ></div>
                    </MarkdownContentPrimitive>
                </div>
            </MarkdownPrimitive>
        }.into_any(),
    }
}
