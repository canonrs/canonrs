use leptos::prelude::*;
use super::code_block_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <CodeBlock 
            code="fn main() {\n    println!(\"Hello, world!\");\n}".to_string()
            language="rust".to_string()
        />
    }
}
