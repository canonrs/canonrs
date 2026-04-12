mod code_block_ui;
#[cfg(feature = "ssr")]
pub(crate) mod highlighter;
#[cfg(feature = "examples")]
pub mod examples;

// no types to re-export from code_block_ui
#[cfg(feature = "examples")]
pub use examples::*;
pub use code_block_ui::CodeBlockPreview;

pub mod preview;
pub use preview::CodeBlockShowcasePreview;
pub mod code_block_boundary;
pub use code_block_boundary::*;
