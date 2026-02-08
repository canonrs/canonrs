//! # CodeBlock Block
//! Syntax-highlighted code display with copy functionality
//! Uses: Primitive (structure) + UI (composition)

use leptos::prelude::*;
use crate::ui::code_block::CodeBlock as CodeBlockUI;

#[component]
pub fn CodeBlockBlock(
    #[prop(into)] code: String,
    #[prop(optional, into)] language: Option<String>,
    #[prop(default = true)] show_copy: bool,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let code_ui = if let Some(lang) = language {
        view! {
            <CodeBlockUI 
                code=code.clone()
                language=lang
                show_copy=show_copy
            />
        }.into_any()
    } else {
        view! {
            <CodeBlockUI 
                code=code.clone()
                show_copy=show_copy
            />
        }.into_any()
    };

    view! {
        <div class=format!("canon-code-block-wrapper {}", class) data-block="code-block">
            {code_ui}
        </div>
    }
}
