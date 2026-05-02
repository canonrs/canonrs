//! CanonRS Core — shared types, providers, primitives


pub mod infra;
pub mod canvas_state;


pub mod primitives;
pub use primitives::*;
pub mod meta;
pub mod meta_types;
pub mod catalog_types;
pub use catalog_types::*;
pub mod block_types;
pub use block_types::*;
pub use meta::*;
pub use canonrs_style::style_contract;
pub use canonrs_style::*;

pub mod generated;
pub mod prelude;

pub use crate::primitives::{TocItem, Orientation, NavigationState, HeadingHierarchy, HeadingNode};
