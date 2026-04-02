pub mod surface;
pub mod actions;
pub mod states;

pub use surface::SEMANTICS_SURFACE;
pub use actions::SEMANTICS_ACTIONS;
pub use states::SEMANTICS_STATES;

// Compatibilidade com semantic_generator.rs
pub struct SemanticBridge {
    pub token: &'static str,
    pub theme_ref: &'static str,
}

pub const SEMANTIC_BRIDGES: &[SemanticBridge] = &[
    SemanticBridge { token: "color-background",          theme_ref: "var(--theme-surface-bg)" },
    SemanticBridge { token: "color-foreground",          theme_ref: "var(--theme-surface-fg)" },
    SemanticBridge { token: "color-muted",               theme_ref: "var(--theme-surface-muted)" },
    SemanticBridge { token: "color-muted-foreground",    theme_ref: "var(--theme-surface-fg-muted)" },
    SemanticBridge { token: "color-border",              theme_ref: "var(--theme-surface-border)" },
    SemanticBridge { token: "color-primary",             theme_ref: "var(--theme-action-primary-bg)" },
    SemanticBridge { token: "color-primary-foreground",  theme_ref: "var(--theme-action-primary-fg)" },
    SemanticBridge { token: "color-secondary",           theme_ref: "var(--theme-action-secondary-bg)" },
    SemanticBridge { token: "color-secondary-foreground",theme_ref: "var(--theme-action-secondary-fg)" },
    SemanticBridge { token: "color-accent",              theme_ref: "var(--theme-action-accent-bg)" },
    SemanticBridge { token: "color-accent-foreground",   theme_ref: "var(--theme-action-accent-fg)" },
    SemanticBridge { token: "color-success",             theme_ref: "var(--theme-state-success-bg)" },
    SemanticBridge { token: "color-success-foreground",  theme_ref: "var(--theme-state-success-fg)" },
    SemanticBridge { token: "color-warning",             theme_ref: "var(--theme-state-warning-bg)" },
    SemanticBridge { token: "color-warning-foreground",  theme_ref: "var(--theme-state-warning-fg)" },
    SemanticBridge { token: "color-error",               theme_ref: "var(--theme-state-error-bg)" },
    SemanticBridge { token: "color-error-foreground",    theme_ref: "var(--theme-state-error-fg)" },
    SemanticBridge { token: "color-info",                theme_ref: "var(--theme-state-info-bg)" },
    SemanticBridge { token: "color-info-foreground",     theme_ref: "var(--theme-state-info-fg)" },
    SemanticBridge { token: "color-popover",             theme_ref: "var(--theme-overlay-bg)" },
    SemanticBridge { token: "color-popover-foreground",  theme_ref: "var(--theme-overlay-fg)" },
    SemanticBridge { token: "color-destructive",         theme_ref: "var(--theme-state-error-bg)" },
    SemanticBridge { token: "color-destructive-foreground", theme_ref: "var(--theme-state-error-fg)" },
    SemanticBridge { token: "color-ring",                theme_ref: "var(--theme-action-focus-ring)" },
];
