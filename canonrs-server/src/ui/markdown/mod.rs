mod markdown_ui;
pub mod markdown_boundary;
pub mod preview;

pub use markdown_boundary::*;
pub use markdown_boundary::{MarkdownSurface, MarkdownLayout, MarkdownContent, MarkdownTOC};
pub use preview::MarkdownShowcasePreview;

#[cfg(feature = "ssr")]
#[cfg(feature = "ssr")]
#[cfg(not(feature = "ssr"))]
#[cfg(not(feature = "ssr"))]
pub use markdown_ui::{RenderedMarkdown, TocPosition};

#[cfg(feature = "ssr")]
mod renderer;
#[cfg(feature = "ssr")]
mod toc_extractor;

pub use markdown_ui::RenderedMarkdown;
pub use markdown_ui::TocPosition;

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
