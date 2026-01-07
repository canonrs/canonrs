#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeToggleVariant {
    Icon,      // ☀️ 🌙 💻 (default)
    Button,    // "Light / Dark"
    Dropdown,  // Light / Dark / System
}

impl Default for ThemeToggleVariant {
    fn default() -> Self {
        Self::Icon
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleSize {
    Sm,
    Md,
    Lg,
}

impl Default for ToggleSize {
    fn default() -> Self {
        Self::Md
    }
}
