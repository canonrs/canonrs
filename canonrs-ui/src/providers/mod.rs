//! Provider re-exports from canonrs-providers
//! 
//! Mantido para compatibilidade, mas providers agora vivem em crate separado

pub use canonrs_providers::theme::{ThemeProvider, ThemeContext, ThemeMode};
pub use canonrs_providers::language::LanguageProvider;
pub use canonrs_providers::density::{DensityProvider, DensityContext, DensityLevel};

// Providers locais que ainda não foram migrados
pub mod canonrs_root;
pub use canonrs_root::CanonRSRoot;

mod layout_provider;
mod layout_types;
mod sidebar_provider;

pub use layout_provider::*;
pub use layout_types::*;
pub use sidebar_provider::*;
