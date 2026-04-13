use leptos::prelude::*;
use super::code_block_boundary::CodeBlock;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CodeBlockShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <CodeBlock
                code="fn main() {\n    println!(\"Hello, world!\");\n}".to_string()
                language="rust".to_string()
                show_copy=true
            />
            <p data-rs-showcase-preview-anchor="">
                "SSR-safe syntax highlighting with deterministic DOM output."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With filename"</span>
                <CodeBlock
                    code="const greet = (name: string) => `Hello, ${name}!`;".to_string()
                    language="typescript".to_string()
                    filename="greet.ts".to_string()
                    show_copy=true
                />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With line numbers"</span>
                <CodeBlock
                    code="[dependencies]\nleptos = { version = \"0.8\" }\ncanorans = { version = \"0.1\" }".to_string()
                    language="toml".to_string()
                    show_line_numbers=true
                />
            </Stack>
        </Stack>
    }
}
