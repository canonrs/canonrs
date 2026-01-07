pub mod animations;
pub mod border;
pub mod colors;
pub mod contextual;
pub mod motion;
pub mod radius;
pub mod semantic;
pub mod shadows;
pub mod spacing;
pub mod state;
pub mod typography;
pub mod z_index;

// Re-exports
pub use animations::AnimationVariant;
pub use border::*;
pub use colors::*;
pub use contextual::*;
pub use motion::*;
pub use radius::*;
pub use semantic::{SEMANTIC, SemanticColors};
pub use shadows::*;
pub use spacing::{SPACING, SpacingTokens};
pub use state::*;
pub use typography::*;
pub use z_index::*;

// Language support
pub struct Language {
    pub code: &'static str,
    pub name: &'static str,
    pub flag: &'static str,
}

pub const LANGUAGES: [Language; 2] = [
    Language { code: "pt", name: "Português", flag: "🇧🇷" },
    Language { code: "en", name: "English", flag: "🇺🇸" },
];

// Theme support
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

pub const THEME_TOKENS_ARRAY: [(&str, &str); 3] = [
    ("light", "Light"),
    ("dark", "Dark"),
    ("system", "System"),
];

// Density support
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Density {
    Compact,
    Normal,
    Comfortable,
}

// Chart colors
// Form validation
pub struct EmailValidation;
impl EmailValidation {
    pub fn error_message(email: &str) -> Option<&'static str> {
        if email.is_empty() {
            Some("Email is required")
        } else if !email.contains('@') {
            Some("Invalid email format")
        } else {
            None
        }
    }
}

pub struct PasswordValidation;
impl PasswordValidation {
    pub fn error_message(password: &str) -> Option<&'static str> {
        if password.is_empty() {
            Some("Password is required")
        } else if password.len() < 8 {
            Some("Password must be at least 8 characters")
        } else {
            None
        }
    }
}

// Theme tokens struct
pub struct ThemeTokens {
    pub icon_size: &'static str,
}

pub const THEME_TOKENS_CONFIG: ThemeTokens = ThemeTokens {
    icon_size: "1.5rem",
};

// Chart colors struct
pub struct ChartColors {
    pub chart_1: &'static str,
    pub chart_2: &'static str,
    pub chart_3: &'static str,
    pub chart_4: &'static str,
    pub chart_5: &'static str,
}

pub const CHART_COLORS: ChartColors = ChartColors {
    chart_1: "hsl(var(--chart-1))",
    chart_2: "hsl(var(--chart-2))",
    chart_3: "hsl(var(--chart-3))",
    chart_4: "hsl(var(--chart-4))",
    chart_5: "hsl(var(--chart-5))",
};

// Canon Rule compliance mapping
pub mod canon_mapping;
