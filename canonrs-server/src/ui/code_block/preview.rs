use leptos::prelude::*;
use super::code_block_boundary::CodeBlock;

#[component]
pub fn CodeBlockShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <CodeBlock
                    code="fn main() {\n    println!(\"Hello, world!\");\n}".to_string()
                    language="rust".to_string()
                    show_copy=true
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "SSR-safe syntax highlighting with deterministic DOM output."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With filename"</span>
                <div data-rs-showcase-preview-row="" style="width:100%;">
                    <CodeBlock
                        code="const greet = (name: string) => `Hello, ${name}!`;".to_string()
                        language="typescript".to_string()
                        filename="greet.ts".to_string()
                        show_copy=true
                    />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With line numbers"</span>
                <div data-rs-showcase-preview-row="" style="width:100%;">
                    <CodeBlock
                        code="[dependencies]\nleptos = { version = \"0.8\" }\ncanorans = { version = \"0.1\" }".to_string()
                        language="toml".to_string()
                        show_line_numbers=true
                    />
                </div>
            </div>
        </div>
    }
}
