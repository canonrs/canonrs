pub mod avatar_ui;
#[cfg(feature = "examples")]
pub mod examples;

pub use avatar_ui::{AvatarShape, AvatarSize, AvatarStatus};
#[cfg(feature = "examples")]
pub use examples::*;

pub use avatar_ui::AvatarPreview;

pub mod preview;
pub use preview::AvatarShowcasePreview;
pub mod avatar_boundary;
pub use avatar_boundary::*;
