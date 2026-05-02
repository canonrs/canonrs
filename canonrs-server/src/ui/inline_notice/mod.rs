mod inline_notice_ui;
pub mod inline_notice_boundary;
pub mod preview;

pub use inline_notice_boundary::*;
pub use inline_notice_boundary::InlineNotice;
pub use canonrs_core::primitives::InlineNoticeVariant;
pub use preview::InlineNoticeShowcasePreview;
