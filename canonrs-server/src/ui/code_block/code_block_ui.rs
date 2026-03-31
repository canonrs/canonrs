
use leptos::prelude::*;
use canonrs_core::primitives::{
    CodeBlockPrimitive, CodeBlockHeaderPrimitive, CodeBlockLanguagePrimitive,
    CodeBlockFilenamePrimitive,
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
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    let pre_html = {
        let result = highlight(&code, &language);
        result.lines.into_iter().enumerate().map(|(i, html)| {
            format!(r#"<span data-rs-code-line="" data-rs-line-number="{}">{}</span>"#, i + 1, html)
        }).collect::<Vec<_>>().join("")
    };
    #[cfg(not(feature = "ssr"))]
    let pre_html = String::new();

    let lang_display = language.clone();
    let copy_id = id.as_deref().map(|i| format!("{}-copy", i));

    view! {
        <CodeBlockPrimitive
            language=lang_display.clone()
            class=class
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
                    <CopyButton text=code.clone() id=copy_id.unwrap_or_default() />
                })}
            </CodeBlockHeaderPrimitive>
            <pre data-rs-code-pre="" inner_html=pre_html />
        </CodeBlockPrimitive>
    }
}

#[component]
pub fn CodeBlockPreview() -> impl IntoView {
    view! { <CodeBlock code="fn main() {}".to_string() language="rust".to_string() /> }
}
