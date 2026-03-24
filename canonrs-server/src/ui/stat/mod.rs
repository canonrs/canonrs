pub mod stat_ui;
#[cfg(feature = "examples")]
pub mod examples;

pub use stat_ui::{
    Stat, StatSize, StatTrend, StatAlign,
    StatHeader, StatBody, StatValue, StatLabel, StatDelta, StatIcon
};
#[cfg(feature = "examples")]
pub use examples::*;

pub use stat_ui::StatPreview;
