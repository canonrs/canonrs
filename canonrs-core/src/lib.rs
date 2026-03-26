//! CanonRS Core — shared types, providers, primitives

pub mod shared;
pub use shared::*;

pub mod design;

pub mod density;
pub mod hydration;
pub mod language;
pub mod root;
pub mod theme;
pub mod prelude;

pub use navigation_context::{NavigationState, HeadingHierarchy, HeadingNode};

pub mod primitives;
pub use primitives::*;
pub mod utils;
pub mod state_engine;
pub mod dom_contract;
pub mod behavior_engine;
pub use state_engine::*;
pub use behavior_engine::*;
pub mod meta;
pub use meta::*;
pub mod style_contract;
pub mod styling;
pub use style_contract::*;
