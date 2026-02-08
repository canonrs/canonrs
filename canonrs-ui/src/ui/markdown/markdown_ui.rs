use leptos::prelude::*;
use crate::primitives::markdown::*;

#[derive(Clone, Debug)]
pub struct RenderedMarkdown {
    pub html: String,
    pub toc: Vec<TocItem>,
}

#[derive(Clone, Debug)]
pub struct TocItem {
    pub id: String,
    pub title: String,
    pub level: u8,
}

#[component]
pub fn MarkdownSurface(
    rendered: RenderedMarkdown,
    #[prop(default = true)] show_toc: bool,
) -> impl IntoView {
    let has_toc = !rendered.toc.is_empty() && show_toc;
    let rendered = StoredValue::new(rendered);

    view! {
        <MarkdownPrimitive>
            <Show when=move || has_toc>
                <MarkdownTocPrimitive>
                    <h3 data-md-toc-title="">"Contents"</h3>
                    <ul data-md-toc-list="">
                        {move || rendered.get_value().toc.iter().map(|item| {
                            let title = item.title.clone();
                            let href = format!("#{}", item.id);
                            let depth = item.level.to_string();
                            view! { <li attr:data-markdown-toc-item="" attr:data-depth=depth><a href=href data-md-toc-link="">{title}</a></li> }
                        }).collect_view()}
                    </ul>
                </MarkdownTocPrimitive>
            </Show>
            <MarkdownContentPrimitive>
                <div data-md-content="" inner_html={move || rendered.get_value().html}></div>
            </MarkdownContentPrimitive>
        </MarkdownPrimitive>
    }
}
