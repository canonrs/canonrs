//! Shared types between SSR and CSR

pub mod shared;

pub use shared::*;

pub mod design;
pub use navigation_context::{NavigationState, HeadingHierarchy, HeadingNode};
