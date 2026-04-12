use leptos::prelude::*;
use super::code_block_ui::CodeBlock as CodeBlockUi;

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
    view! {
        <CodeBlockUi
            code=code
            language=language
            filename=filename.unwrap_or_default()
            show_copy=show_copy
            show_line_numbers=show_line_numbers
            class=class
            id=id.unwrap_or_default()
        />
};
}
