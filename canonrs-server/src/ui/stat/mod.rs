pub mod stat_ui;
pub mod stat_island;
pub use stat_island::{
    StatIsland, StatHeaderIsland, StatBodyIsland,
    StatValueIsland, StatLabelIsland, StatDeltaIsland, StatIconIsland,
};
#[cfg(feature = "examples")]
pub mod examples;

pub use stat_ui::{
    Stat, StatSize, StatTrend, StatAlign,
    StatHeader, StatBody, StatValue, StatLabel, StatDelta, StatIcon
};
#[cfg(feature = "examples")]
pub use examples::*;

pub use stat_ui::StatPreview;

pub mod preview;
pub use preview::StatShowcasePreview;
