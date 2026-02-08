// SEMANTIC LAYER — CanonRS Bridge
//
// Purpose: Map friendly semantic names (--color-*) to theme tokens (--theme-*)
// This layer exists ONLY for backwards compatibility and DX.
//
// Layering:
// Primitive  -> HSL values
// Theme      -> --theme-* (normalized vocabulary, changes with .dark)
// Semantic   -> --color-* (friendly aliases, maps to theme)
// Family     -> component tokens
//
// Rule: Semantic NEVER generates HSL. It only bridges names.

pub struct SemanticBridge {
    pub token: &'static str,
    pub theme_ref: &'static str,
}

pub const SEMANTIC_BRIDGES: &[SemanticBridge] = &[
    
    // ======================================================================
    // SURFACE — Root & containers
    // ======================================================================
    
    SemanticBridge {
        token: "color-background",
        theme_ref: "var(--theme-surface-bg)",
    },
    SemanticBridge {
        token: "color-foreground",
        theme_ref: "var(--theme-surface-fg)",
    },
    SemanticBridge {
        token: "color-muted",
        theme_ref: "var(--theme-surface-muted)",
    },
    SemanticBridge {
        token: "color-muted-foreground",
        theme_ref: "var(--theme-surface-fg-muted)",
    },
    SemanticBridge {
        token: "color-border",
        theme_ref: "var(--theme-surface-border)",
    },

    // ======================================================================
    // ACTIONS — Interactive elements
    // ======================================================================
    
    SemanticBridge {
        token: "color-primary",
        theme_ref: "var(--theme-action-primary-bg)",
    },
    SemanticBridge {
        token: "color-primary-foreground",
        theme_ref: "var(--theme-action-primary-fg)",
    },
    SemanticBridge {
        token: "color-secondary",
        theme_ref: "var(--theme-action-secondary-bg)",
    },
    SemanticBridge {
        token: "color-secondary-foreground",
        theme_ref: "var(--theme-action-secondary-fg)",
    },
    SemanticBridge {
        token: "color-accent",
        theme_ref: "var(--theme-action-accent-bg)",
    },
    SemanticBridge {
        token: "color-accent-foreground",
        theme_ref: "var(--theme-action-accent-fg)",
    },

    // ======================================================================
    // STATES — Feedback & status
    // ======================================================================
    
    SemanticBridge {
        token: "color-success",
        theme_ref: "var(--theme-state-success-bg)",
    },
    SemanticBridge {
        token: "color-success-foreground",
        theme_ref: "var(--theme-state-success-fg)",
    },
    SemanticBridge {
        token: "color-warning",
        theme_ref: "var(--theme-state-warning-bg)",
    },
    SemanticBridge {
        token: "color-warning-foreground",
        theme_ref: "var(--theme-state-warning-fg)",
    },
    SemanticBridge {
        token: "color-error",
        theme_ref: "var(--theme-state-error-bg)",
    },
    SemanticBridge {
        token: "color-error-foreground",
        theme_ref: "var(--theme-state-error-fg)",
    },
    SemanticBridge {
        token: "color-info",
        theme_ref: "var(--theme-state-info-bg)",
    },
    SemanticBridge {
        token: "color-info-foreground",
        theme_ref: "var(--theme-state-info-fg)",
    },

    // ======================================================================
    // OVERLAYS — Floating surfaces
    // ======================================================================
    
    SemanticBridge {
        token: "color-popover",
        theme_ref: "var(--theme-overlay-bg)",
    },
    SemanticBridge {
        token: "color-popover-foreground",
        theme_ref: "var(--theme-overlay-fg)",
    },

    // ======================================================================
    // LEGACY ALIASES (deprecated, will be removed)
    // ======================================================================
    
    SemanticBridge {
        token: "color-destructive",
        theme_ref: "var(--theme-state-error-bg)",
    },
    SemanticBridge {
        token: "color-destructive-foreground",
        theme_ref: "var(--theme-state-error-fg)",
    },
];

pub const SEMANTIC_TOKEN_NAMES: &[&str] = &[
    "color-background",
    "color-foreground",
    "color-muted",
    "color-muted-foreground",
    "color-border",
    "color-primary",
    "color-primary-foreground",
    "color-secondary",
    "color-secondary-foreground",
    "color-accent",
    "color-accent-foreground",
    "color-success",
    "color-success-foreground",
    "color-warning",
    "color-warning-foreground",
    "color-error",
    "color-error-foreground",
    "color-info",
    "color-info-foreground",
    "color-popover",
    "color-popover-foreground",
    "color-destructive",
    "color-destructive-foreground",
];
