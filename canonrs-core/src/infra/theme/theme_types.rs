use leptos::prelude::*;

/// Theme mode enum
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

/// Theme context
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
    pub preset: RwSignal<String>,
}

impl ThemeContext {
    pub fn is_dark(&self) -> bool {
        self.mode.get() == ThemeMode::Dark
    }

    pub fn toggle(&self) {
        self.mode.update(|mode| {
            *mode = match *mode {
                ThemeMode::Dark => ThemeMode::Light,
                _ => ThemeMode::Dark,
            };
        });
    }
}
