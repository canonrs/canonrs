use crate::design::tokens::FamilyToken;

/// CORE TOKENS — Foundation Layer
/// Base primitives that all families reference
/// Scope: Immutable design constants

pub const CORE_TOKENS: &[FamilyToken] = &[
    // === COLORS BASE ===
    FamilyToken::new("color-background", "hsl(0 0% 100%)"),
    FamilyToken::new("color-foreground", "hsl(222.2 84% 4.9%)"),
    FamilyToken::new("color-primary", "hsl(222.2 47.4% 11.2%)"),
    FamilyToken::new("color-primary-foreground", "hsl(210 40% 98%)"),
    FamilyToken::new("color-accent", "hsl(210 40% 96.1%)"),
    FamilyToken::new("color-accent-foreground", "hsl(222.2 47.4% 11.2%)"),
    FamilyToken::new("color-muted", "hsl(210 40% 96.1%)"),
    FamilyToken::new("color-muted-foreground", "hsl(215.4 16.3% 46.9%)"),
    FamilyToken::new("color-destructive", "hsl(0 84.2% 60.2%)"),
    FamilyToken::new("color-destructive-foreground", "hsl(210 40% 98%)"),
    FamilyToken::new("color-border", "hsl(214.3 31.8% 91.4%)"),
    FamilyToken::new("color-input", "hsl(214.3 31.8% 91.4%)"),
    FamilyToken::new("color-card", "hsl(0 0% 100%)"),
    FamilyToken::new("color-card-foreground", "hsl(222.2 84% 4.9%)"),

    // === COLORS EXTENDED (Enterprise) ===
    FamilyToken::new("color-secondary", "hsl(210 40% 96.1%)"),
    FamilyToken::new("color-ring", "hsl(222.2 84% 4.9%)"),
    FamilyToken::new("color-popover", "hsl(0 0% 100%)"),
    FamilyToken::new("color-popover-foreground", "hsl(222.2 84% 4.9%)"),
    
    FamilyToken::new("color-success", "hsl(142 76% 36%)"),
    FamilyToken::new("color-warning", "hsl(38 92% 50%)"),
    FamilyToken::new("color-info", "hsl(199 89% 48%)"),
    
    FamilyToken::new("color-primary-hover", "hsl(222.2 47.4% 8%)"),
    FamilyToken::new("color-primary-active", "hsl(222.2 47.4% 5%)"),
    
    FamilyToken::new("color-border-muted", "hsl(214.3 31.8% 95%)"),
    FamilyToken::new("color-success-border", "hsl(142 76% 36%)"),
    FamilyToken::new("color-success-fg", "hsl(142 76% 36%)"),

    // === SEMANTIC SURFACES (theme-resolved) ===
    FamilyToken::new("semantic-surface-elevated", "var(--theme-surface-elevated)"),
    FamilyToken::new("semantic-border-subtle", "var(--theme-border-subtle)"),
    FamilyToken::new("semantic-info-surface", "var(--theme-info-surface)"),
    FamilyToken::new("semantic-success-surface", "var(--theme-success-surface)"),
    FamilyToken::new("semantic-warning-surface", "var(--theme-warning-surface)"),
    FamilyToken::new("semantic-error-surface", "var(--theme-error-surface)"),

    // === SPACING ===
    FamilyToken::new("space-2xs", "0.125rem"),
    FamilyToken::new("space-xs", "0.25rem"),
    FamilyToken::new("space-sm", "0.5rem"),
    FamilyToken::new("space-md", "1rem"),
    FamilyToken::new("space-lg", "1.5rem"),
    FamilyToken::new("space-xl", "2rem"),
    FamilyToken::new("space-2xl", "3rem"),
    FamilyToken::new("space-3xl", "4rem"),

    // === RADIUS ===
    FamilyToken::new("radius-sm", "0.25rem"),
    FamilyToken::new("radius-md", "0.5rem"),
    FamilyToken::new("radius-lg", "0.75rem"),
    FamilyToken::new("radius-xl", "1rem"),
    FamilyToken::new("radius-full", "9999px"),

    // === TYPOGRAPHY ===
    FamilyToken::new("font-family-sans", "system-ui, -apple-system, sans-serif"),
    FamilyToken::new("font-family-mono", "ui-monospace, monospace"),
    
    FamilyToken::new("font-size-xs", "0.75rem"),
    FamilyToken::new("font-size-sm", "0.875rem"),
    FamilyToken::new("font-size-md", "0.9375rem"),
    FamilyToken::new("font-size-base", "1rem"),
    FamilyToken::new("font-size-lg", "1.125rem"),
    FamilyToken::new("font-size-xl", "1.25rem"),
    FamilyToken::new("font-size-2xl", "1.5rem"),
    FamilyToken::new("font-size-3xl", "1.875rem"),
    FamilyToken::new("font-size-4xl", "2.25rem"),

    FamilyToken::new("font-weight-regular", "400"),
    FamilyToken::new("font-weight-normal", "400"),
    FamilyToken::new("font-weight-medium", "500"),
    FamilyToken::new("font-weight-semibold", "600"),
    FamilyToken::new("font-weight-bold", "700"),

    FamilyToken::new("line-height-tight", "1.25"),
    FamilyToken::new("line-height-normal", "1.5"),
    FamilyToken::new("line-height-relaxed", "1.75"),

    // === LAYOUT ===
    FamilyToken::new("layout-width-sm", "16rem"),
    FamilyToken::new("layout-width-md", "20rem"),
    FamilyToken::new("layout-width-lg", "32rem"),
    FamilyToken::new("layout-width-xl", "48rem"),
    FamilyToken::new("layout-width-2xl", "64rem"),
    FamilyToken::new("layout-width-full", "80rem"),
    
    FamilyToken::new("layout-sidebar-width", "16rem"),
    FamilyToken::new("layout-sidebar-width-collapsed", "4rem"),
    FamilyToken::new("layout-content-max-width", "80rem"),
    
    FamilyToken::new("layout-width-dialog", "32rem"),
    FamilyToken::new("layout-width-toc", "15rem"),
    FamilyToken::new("layout-height-header", "4rem"),
    
    FamilyToken::new("border-width-hairline", "0.5px"),
    FamilyToken::new("border-width-thin", "1px"),

    // === SHADOWS ===
    FamilyToken::new("shadow-sm", "0 1px 2px 0 rgb(0 0 0 / 0.05)"),
    FamilyToken::new("shadow-md", "0 4px 6px -1px rgb(0 0 0 / 0.1)"),
    FamilyToken::new("shadow-lg", "0 10px 15px -3px rgb(0 0 0 / 0.1)"),
    FamilyToken::new("shadow-xl", "0 20px 25px -5px rgb(0 0 0 / 0.1)"),
    FamilyToken::new("shadow-2xl", "0 25px 50px -12px rgb(0 0 0 / 0.25)"),

    // === MOTION ===
    FamilyToken::new("motion-duration-instant", "100ms"),
    FamilyToken::new("motion-duration-fast", "150ms"),
    FamilyToken::new("motion-duration-normal", "200ms"),
    FamilyToken::new("motion-duration-slow", "300ms"),
    
    FamilyToken::new("motion-ease-standard", "cubic-bezier(0.4, 0.0, 0.2, 1)"),
    FamilyToken::new("motion-ease-decelerate", "cubic-bezier(0.0, 0.0, 0.2, 1)"),
    FamilyToken::new("motion-ease-accelerate", "cubic-bezier(0.4, 0.0, 1, 1)"),

    // === ANIMATE ===
    FamilyToken::new("animate-duration-fast", "150ms"),
    FamilyToken::new("animate-duration-normal", "300ms"),
    FamilyToken::new("animate-duration-slow", "500ms"),
    FamilyToken::new("animate-delay-none", "0ms"),
    FamilyToken::new("animate-delay-short", "150ms"),
    FamilyToken::new("animate-delay-long", "300ms"),

    // === COMPONENT DIMENSIONS ===
    FamilyToken::new("size-input-sm", "2rem"),
    FamilyToken::new("size-input-lg", "3rem"),
    FamilyToken::new("size-avatar-lg", "3rem"),
    FamilyToken::new("size-avatar-xl", "4rem"),
    FamilyToken::new("size-icon-lg", "3rem"),
    FamilyToken::new("size-button-lg", "3rem"),
    FamilyToken::new("size-cell", "2.25rem"),
    FamilyToken::new("size-nav-item", "2.5rem"),
    FamilyToken::new("size-pagination", "2.5rem"),
    FamilyToken::new("size-color-picker", "10rem"),
    FamilyToken::new("size-color-swatch", "2rem"),
    FamilyToken::new("size-switch-width", "2.75rem"),
    FamilyToken::new("size-switch-translate", "1.375rem"),
    FamilyToken::new("size-textarea-min", "5rem"),

    // === SEMANTIC ACTIONS (theme-resolved) ===
    FamilyToken::new("semantic-action-primary-bg", "var(--theme-action-primary-bg)"),
    FamilyToken::new("semantic-action-primary-fg", "var(--theme-action-primary-fg)"),
    FamilyToken::new("semantic-action-primary-hover", "var(--theme-action-primary-hover)"),
    FamilyToken::new("semantic-action-primary-active", "var(--theme-action-primary-active)"),
];
