pub mod inline_notice_ui;
pub mod inline_notice_boundary;
pub use inline_notice_boundary::*;
pub mod preview;
// no types to re-export from inline_notice_ui
pub use inline_notice_boundary::InlineNotice;
pub use canonrs_core::primitives::InlineNoticeVariant;

#[cfg(feature = "examples")]
pub mod examples;
#[cfg(feature = "examples")]
pub use examples::*;

pub use inline_notice_ui::InlineNoticePreview;
pub use preview::InlineNoticeShowcasePreview;
