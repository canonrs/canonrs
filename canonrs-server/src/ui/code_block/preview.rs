use leptos::prelude::*;
use super::code_block_boundary::CodeBlock;
use crate::blocks::card::CardBlock;
use crate::ui::card::{CardHeader, CardTitle, CardContent};
use canonrs_core::slot;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn CodeBlockShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <CardBlock
                header=slot!(|| view! {
                    <CardHeader><CardTitle>"Rust"</CardTitle></CardHeader>
                }.into_any())
                content=slot!(|| view! {
                    <CardContent>
                        <CodeBlock
                            code="fn main() {\n    println!(\"Hello, world!\");\n}".to_string()
                            language="rust".to_string()
                            show_copy=true
                        />
                    </CardContent>
                }.into_any())
            />
            <p data-rs-showcase-preview-anchor="">
                "SSR-safe syntax highlighting with deterministic DOM output."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With filename"</span>
                <CardBlock
                    content=slot!(|| view! {
                        <CardContent>
                            <CodeBlock
                                code="const greet = (name: string) => `Hello, ${name}!`;".to_string()
                                language="typescript".to_string()
                                filename="greet.ts".to_string()
                                show_copy=true
                            />
                        </CardContent>
                    }.into_any())
                />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With line numbers"</span>
                <CardBlock
                    content=slot!(|| view! {
                        <CardContent>
                            <CodeBlock
                                code="[dependencies]\nleptos = { version = \"0.8\" }".to_string()
                                language="toml".to_string()
                                show_line_numbers=true
                            />
                        </CardContent>
                    }.into_any())
                />
            </Stack>
        </Stack>
    }
}
