use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::System
    }
}

#[derive(Clone)]
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
    pub preset: RwSignal<String>,
}

// Hook para usar tema
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("ThemeContext not found. Make sure ThemeProvider wraps your app.")
}
