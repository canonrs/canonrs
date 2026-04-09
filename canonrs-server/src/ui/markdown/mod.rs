pub mod markdown_ui;
pub mod markdown_toolbar;
#[cfg(feature = "ssr")]
mod renderer;
#[cfg(feature = "ssr")]
mod toc_extractor;
#[cfg(feature = "examples")]
pub mod examples;

pub use markdown_ui::*;
pub use markdown_toolbar::*;
pub use markdown_ui::MarkdownPreview;
pub use markdown_ui::RenderedMarkdown;
#[cfg(feature = "examples")]
pub use examples::markdown_basic_example;

#[cfg(feature = "ssr")]
pub use toc_extractor::{TocExtractor, slugify};

#[cfg(feature = "ssr")]
pub fn render_markdown(md: &str) -> RenderedMarkdown {
    renderer::render_markdown(md)
}

#[cfg(not(feature = "ssr"))]
pub fn render_markdown(_: &str) -> RenderedMarkdown { RenderedMarkdown::default() }

#[cfg(feature = "ssr")]
pub fn render_markdown_with_prefix(md: &str, prefix: &str) -> RenderedMarkdown {
    renderer::render_markdown_with_prefix(md, prefix)
}
#[cfg(not(feature = "ssr"))]
pub fn render_markdown_with_prefix(_: &str, _: &str) -> RenderedMarkdown { RenderedMarkdown::default() }

pub fn render_with_prefix(md: &str, prefix: &str) -> RenderedMarkdown {
    render_markdown_with_prefix(md, prefix)
}

pub mod preview;
pub use preview::MarkdownShowcasePreview;
pub mod markdown_island;
pub use markdown_island::{MarkdownSurfaceIsland, MarkdownLayoutIsland, MarkdownContentIsland, MarkdownTOCIsland};
