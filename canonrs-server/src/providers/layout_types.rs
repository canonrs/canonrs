use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutMode {
    Sidebar,
    Topbar,
    Minimal,
}

impl Default for LayoutMode {
    fn default() -> Self {
        Self::Sidebar
    }
}

impl LayoutMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sidebar => "sidebar",
            Self::Topbar => "topbar",
            Self::Minimal => "minimal",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "topbar" => Self::Topbar,
            "minimal" => Self::Minimal,
            _ => Self::Sidebar,
        }
    }
}

impl ToString for LayoutMode {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

#[derive(Clone, Copy)]
pub struct LayoutContext {
    pub mode: RwSignal<LayoutMode>,
}

impl LayoutContext {
    pub fn mode(&self) -> LayoutMode {
        self.mode.get()
    }

    pub fn set_mode(&self, value: String) {
        self.mode.set(LayoutMode::from_str(&value));
    }
}

pub fn use_layout() -> LayoutContext {
    use_context::<LayoutContext>()
        .expect("LayoutContext must be provided")
}
