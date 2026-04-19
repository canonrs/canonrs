//! @canon-level: strict
//! CodeBlock Boundary — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::code_block_ui::CodeBlock as CodeBlockUi;

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(into, default = "text".to_string())] language: String,
    #[prop(into, optional)] filename: Option<String>,
    #[prop(default = false)] show_line_numbers: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CodeBlockUi
            code=code
            language=language
            filename=filename.unwrap_or_default()
            show_line_numbers=show_line_numbers
            class=class
        />
    }
}
