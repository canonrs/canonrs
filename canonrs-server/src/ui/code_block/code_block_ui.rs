//! @canon-level: strict
//! @canon-owner: ui-team
//! CodeBlock UI Component - DevTools level, SSR highlight via syntect

use leptos::prelude::*;
use canonrs_core::primitives::{
    CodeBlockPrimitive, CodeBlockHeaderPrimitive, CodeBlockLanguagePrimitive,
    CodeBlockFilenamePrimitive, CodeBlockPrePrimitive, CodeBlockLinePrimitive,
};
use crate::ui::copy_button::CopyButton;
#[cfg(feature = "ssr")]
use super::highlighter::highlight;

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(into, default = "text".to_string())] language: String,
    #[prop(into, optional)] filename: Option<String>,
    #[prop(default = true)] show_copy: bool,
    #[prop(default = false)] show_line_numbers: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let lines = {
        let result = highlight(&code, &language);
        result.lines
    };
    #[cfg(not(feature = "ssr"))]
    let lines: Vec<String> = vec![code.clone()];

    let lang_display = language.clone();
    let copy_id = format!("{}-copy", id);

    view! {
        <CodeBlockPrimitive
            language=lang_display.clone()
            class=class
            id=id
            attr:data-line-numbers={show_line_numbers.then(|| "true")}
        >
            <CodeBlockHeaderPrimitive>
                <div data-code-header-left="">
                    <CodeBlockLanguagePrimitive language=lang_display />
                    {filename.map(|f| view! {
                        <CodeBlockFilenamePrimitive filename=f />
                    })}
                </div>
                {show_copy.then(|| view! {
                    <CopyButton text=code.clone() id=copy_id />
                })}
            </CodeBlockHeaderPrimitive>

            <CodeBlockPrePrimitive>
                {lines.into_iter().enumerate().map(|(i, html)| {
                    view! {
                        <CodeBlockLinePrimitive
                            html=html
                            line_number={i + 1}
                        />
                    }
                }).collect::<Vec<_>>()}
            </CodeBlockPrePrimitive>
        </CodeBlockPrimitive>
    }
}

#[component]
pub fn CodeBlockPreview() -> impl IntoView {
    view! { <CodeBlock code="fn main() {}".to_string() language="rust".to_string() /> }
}
