//! # CanonMarkdown Component
//! Full-featured markdown renderer

use leptos::prelude::*;
use pulldown_cmark::{TagEnd, Parser, Event, Tag, CodeBlockKind};
use crate::ui::code_block::CodeBlock;
use crate::ui::table_of_contents::TableOfContents;
use crate::utils::markdown::TocExtractor;

#[component]
pub fn CanonMarkdown(
    content: String,
    #[prop(default = true)] show_toc: bool,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let class = StoredValue::new(class.unwrap_or_default());
    let toc_items = if show_toc {
        TocExtractor::extract_toc(&content)
    } else {
        vec![]
    };

    let has_toc = !toc_items.is_empty() && show_toc;
    let components = parse_to_components(&content);

    view! {
        <div
            class={class.get_value()}
            attr:data-canon-markdown=""
            attr:data-has-toc={has_toc}
        >
            {has_toc.then(|| view! {
                <aside attr:data-markdown-toc="">
                    <TableOfContents items={toc_items} />
                </aside>
            })}

            <article attr:data-markdown-content="">
                {components}
            </article>
        </div>
    }
}

fn parse_to_components(content: &str) -> Vec<AnyView> {
    let parser = Parser::new(content);
    let mut result: Vec<AnyView> = Vec::new();
    let mut in_code = false;
    let mut code_lang = String::new();
    let mut code_buf = String::new();
    let mut html_buf = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                if !html_buf.is_empty() {
                    let h = html_buf.clone();
                    result.push(view! { <div inner_html={h}></div> }.into_any());
                    html_buf.clear();
                }
                in_code = true;
                code_lang = lang.to_string();
                code_buf.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code = false;
                let code = code_buf.clone();
                let lang = if code_lang.is_empty() {
                    None
                } else {
                    Some(code_lang.clone())
                };

                result.push(view! {
                    <CodeBlock
                        code={code}
                        language={lang.unwrap_or_default()}
                    />
                }.into_any());
            }
            Event::Text(text) if in_code => {
                code_buf.push_str(&text);
            }
            _ => {
                use pulldown_cmark::html;
                let mut buf = String::new();
                html::push_html(&mut buf, vec![event].into_iter());
                html_buf.push_str(&buf);
            }
        }
    }

    if !html_buf.is_empty() {
        result.push(view! { <div inner_html={html_buf}></div> }.into_any());
    }

    result
}
