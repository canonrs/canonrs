use std::collections::HashMap;
use crate::theme_generator::HSLColor;

pub struct ThemeMapping {
    pub semantic_name: &'static str,
    pub color_source: &'static str,
}

// Mapeamentos configuráveis: --color-* → --theme-*
pub const THEME_MAPPINGS: &[ThemeMapping] = &[
    // Actions
    ThemeMapping { semantic_name: "action-primary-bg", color_source: "primary" },
    ThemeMapping { semantic_name: "action-primary-fg", color_source: "primary-foreground" },
    ThemeMapping { semantic_name: "action-secondary-bg", color_source: "secondary" },
    ThemeMapping { semantic_name: "action-secondary-fg", color_source: "secondary-foreground" },
    ThemeMapping { semantic_name: "action-accent-bg", color_source: "accent" },
    ThemeMapping { semantic_name: "action-accent-fg", color_source: "accent-foreground" },

    // Surfaces
    ThemeMapping { semantic_name: "surface-bg", color_source: "background" },
    ThemeMapping { semantic_name: "surface-fg", color_source: "foreground" },
    ThemeMapping { semantic_name: "surface-border", color_source: "border" },
    ThemeMapping { semantic_name: "surface-elevated", color_source: "card" },
    ThemeMapping { semantic_name: "surface-elevated-fg", color_source: "card-foreground" },
    ThemeMapping { semantic_name: "surface-muted", color_source: "muted" },
    ThemeMapping { semantic_name: "surface-fg-muted", color_source: "muted-foreground" },

    // Overlays
    ThemeMapping { semantic_name: "overlay-bg", color_source: "popover" },
    ThemeMapping { semantic_name: "overlay-fg", color_source: "popover-foreground" },

    // Text
    ThemeMapping { semantic_name: "text-primary", color_source: "foreground" },
    ThemeMapping { semantic_name: "text-secondary", color_source: "muted-foreground" },
    ThemeMapping { semantic_name: "text-muted", color_source: "muted-foreground" },

    // Interactive
    ThemeMapping { semantic_name: "focus-ring", color_source: "ring" },
    ThemeMapping { semantic_name: "input-border", color_source: "input" },

    // Feedback
    ThemeMapping { semantic_name: "success-bg", color_source: "accent" },
    ThemeMapping { semantic_name: "success-fg", color_source: "accent-foreground" },
    ThemeMapping { semantic_name: "success-border", color_source: "accent" },
    ThemeMapping { semantic_name: "error-bg", color_source: "destructive" },
    ThemeMapping { semantic_name: "error-fg", color_source: "destructive-foreground" },
    ThemeMapping { semantic_name: "error-border", color_source: "destructive" },
    ThemeMapping { semantic_name: "warning-bg", color_source: "primary" },
    ThemeMapping { semantic_name: "warning-fg", color_source: "primary-foreground" },
    ThemeMapping { semantic_name: "info-bg", color_source: "muted" },
    ThemeMapping { semantic_name: "info-fg", color_source: "foreground" },

    // State (aliases for feedback - used by badge, toast, alert, etc)
    ThemeMapping { semantic_name: "state-success-bg", color_source: "accent" },
    ThemeMapping { semantic_name: "state-success-fg", color_source: "accent-foreground" },
    ThemeMapping { semantic_name: "state-error-bg", color_source: "destructive" },
    ThemeMapping { semantic_name: "state-error-fg", color_source: "destructive-foreground" },
    ThemeMapping { semantic_name: "state-warning-bg", color_source: "primary" },
    ThemeMapping { semantic_name: "state-warning-fg", color_source: "primary-foreground" },
    ThemeMapping { semantic_name: "state-info-bg", color_source: "muted" },
    ThemeMapping { semantic_name: "state-info-fg", color_source: "foreground" },
];

pub fn generate_theme_tokens(colors: &HashMap<String, HSLColor>) -> String {
    let mut output = String::new();

    for mapping in THEME_MAPPINGS {
        if let Some(color) = colors.get(mapping.color_source) {
            output.push_str(&format!(
                "  --theme-{}: hsl({} {}% {}%);\n",
                mapping.semantic_name, color.h, color.s, color.l
            ));
        }
    }

    output
}
