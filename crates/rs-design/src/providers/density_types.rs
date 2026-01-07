use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DensityMode {
    Compact,
    Comfortable,
    Spacious,
}

impl Default for DensityMode {
    fn default() -> Self {
        Self::Comfortable
    }
}

impl DensityMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Comfortable => "comfortable",
            Self::Spacious => "spacious",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s {
            "compact" => Self::Compact,
            "spacious" => Self::Spacious,
            _ => Self::Comfortable,
        }
    }
    
    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Compact => 0.75,
            Self::Comfortable => 1.0,
            Self::Spacious => 1.25,
        }
    }
}

#[derive(Clone, Copy)]
pub struct DensityContext {
    pub mode: RwSignal<DensityMode>,
}

pub fn use_density() -> DensityContext {
    use_context::<DensityContext>()
        .expect("DensityContext not found. Make sure DensityProvider is in the component tree.")
}
