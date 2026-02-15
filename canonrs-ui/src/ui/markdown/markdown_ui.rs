use leptos::prelude::*;
use canonrs_shared::TocItem;
use crate::primitives::markdown::*;

#[derive(Clone, Debug)]
pub struct RenderedMarkdown {
    pub html: String,
    pub toc: Vec<TocItem>,
}

#[component]
pub fn TableOfContents(
    items: Vec<TocItem>,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <nav data-toc="" id=id>
            <ul data-toc-list="">
                {items.into_iter().map(|item| {
                    let href = format!("#{}", item.id);
                    view! {
                        <MarkdownTocItemPrimitive
                            href=href
                            text=item.text
                            level=item.level
                        />
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </nav>
    }
}

#[component]
pub fn MarkdownSurface(
    rendered: RenderedMarkdown,
    #[prop(default = true)] show_toc: bool,
    #[prop(default = true)] show_toolbar: bool,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    let has_toc = !rendered.toc.is_empty() && show_toc;
    let toc_id = format!("{}-toc", id);
    let rendered = StoredValue::new(rendered);

    view! {
        <MarkdownPrimitive id=id.clone()>
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
                            <ul data-toc-list="">
                                {items.into_iter().map(|item| {
                                    let href = format!("#{}", item.id);
                                    view! {
                                        <MarkdownTocItemPrimitive
                                            href=href
                                            text=item.text
                                            level=item.level
                                        />
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
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
    }
}
