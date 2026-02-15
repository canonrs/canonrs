use leptos::prelude::*;
use super::markdown_ui::MarkdownSurface;
use super::renderer::render_markdown;

pub fn basic_example() -> impl IntoView {
    let markdown = r#"
# CanonRS Markdown

Este é um exemplo de **Markdown** renderizado server-side com `syntect`.

## Código
```rust
fn main() {
    println!("Hello, CanonRS!");
}
```

## Lista

- Item 1
- Item 2
- Item 3

## Tabela

| Nome | Valor |
|------|-------|
| A    | 1     |
| B    | 2     |
"#;

    let rendered = render_markdown(markdown);

    view! {
        <MarkdownSurface rendered=rendered />
    }
}
