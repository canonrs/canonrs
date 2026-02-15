use crate::design::tokens::FamilyToken;

/// FAMILY E — Feedback & Status
/// Components: Alert, AlertDialog, Callout, Badge, Toast, Banner, InlineNotice, EmptyState, ErrorState, StatusDot, Spinner, Progress, Skeleton
/// Scope: User feedback, status indicators, loading states (excludes: state tokens, z-index)

pub const FAMILY_E_FEEDBACK: &[FamilyToken] = &[
    // Feedback foundation
    FamilyToken::new("feedback-color", "var(--theme-action-primary-bg)"),

    // StatusDot - User Presence (NOT semantic feedback)
    FamilyToken::new("status-dot-size", "var(--space-sm)"),
    FamilyToken::new("status-dot-radius", "var(--radius-full)"),
    FamilyToken::new("status-dot-bg-online", "var(--color-success)"),
    FamilyToken::new("status-dot-bg-offline", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("status-dot-bg-away", "var(--color-warning)"),
    FamilyToken::new("status-dot-bg-busy", "var(--theme-state-error-bg)"),
    FamilyToken::new("status-dot-bg-do-not-disturb", "var(--theme-state-error-bg)"),

    // StatusDot - User Presence (NOT semantic feedback)
    FamilyToken::new("feedback-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("feedback-radius", "var(--radius-lg)"),

    // Alert
    FamilyToken::new("alert-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("alert-bg-default", "var(--theme-surface-muted)"),
    FamilyToken::new("alert-bg-info", "var(--theme-state-info-bg)"),
    FamilyToken::new("alert-bg-success", "var(--theme-state-success-bg)"),
    FamilyToken::new("alert-bg-warning", "var(--theme-state-warning-bg)"),
    FamilyToken::new("alert-bg-error", "var(--theme-state-error-bg)"),
    FamilyToken::new("alert-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("alert-border-width", "1px"),
    FamilyToken::new("alert-padding-x", "var(--space-md)"),
    FamilyToken::new("alert-padding-y", "var(--space-sm)"),
    FamilyToken::new("alert-radius", "var(--radius-lg)"),
    FamilyToken::new("alert-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("alert-title-font-size", "var(--font-size-sm)"),
    FamilyToken::new("alert-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("alert-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("alert-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("alert-description-line-height", "var(--line-height-normal)"),
    FamilyToken::new("alert-close-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("alert-close-fg-hover", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("alert-close-size", "var(--space-md)"),

    // Alert Dialog
    FamilyToken::new("alert-dialog-overlay-bg", "rgba(0, 0, 0, 0.5)"),
    FamilyToken::new("alert-dialog-overlay-opacity", "0.5"),
    FamilyToken::new("alert-dialog-overlay-blur", "4px"),
    FamilyToken::new("alert-dialog-content-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("alert-dialog-content-width", "var(--layout-width-dialog)"),
    FamilyToken::new("alert-dialog-content-radius", "var(--radius-lg)"),
    FamilyToken::new("alert-dialog-content-padding", "var(--space-xl)"),
    FamilyToken::new("alert-dialog-content-shadow", "var(--shadow-2xl)"),
    FamilyToken::new("alert-dialog-header-gap", "var(--space-sm)"),
    FamilyToken::new("alert-dialog-header-padding", "0 0 var(--space-md) 0"),
    FamilyToken::new("alert-dialog-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("alert-dialog-title-font-size", "var(--font-size-lg)"),
    FamilyToken::new("alert-dialog-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("alert-dialog-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("alert-dialog-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("alert-dialog-description-line-height", "var(--line-height-relaxed)"),
    FamilyToken::new("alert-dialog-footer-gap", "var(--space-sm)"),
    FamilyToken::new("alert-dialog-footer-padding", "var(--space-md) 0 0 0"),
    FamilyToken::new("alert-dialog-action-gap", "var(--space-sm)"),
    FamilyToken::new("alert-dialog-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("alert-dialog-transition-ease", "var(--motion-ease-standard)"),

    // Callout
    FamilyToken::new("callout-gap", "var(--space-sm)"),
    FamilyToken::new("callout-padding-x", "var(--space-md)"),
    FamilyToken::new("callout-padding-y", "var(--space-sm)"),
    FamilyToken::new("callout-radius", "var(--radius-lg)"),
    FamilyToken::new("callout-border-width", "1px"),
    FamilyToken::new("callout-icon-size", "var(--font-size-xl)"),
    FamilyToken::new("callout-bg-default", "var(--theme-action-secondary-bg)"),
    FamilyToken::new("callout-bg-info", "var(--theme-state-info-bg)"),
    FamilyToken::new("callout-bg-success", "var(--theme-state-success-bg)"),
    FamilyToken::new("callout-bg-warning", "var(--theme-state-warning-bg)"),
    FamilyToken::new("callout-bg-error", "var(--theme-state-error-bg)"),
    FamilyToken::new("callout-border-default", "var(--theme-surface-border)"),
    FamilyToken::new("callout-border-info", "var(--theme-state-info-bg)"),
    FamilyToken::new("callout-border-success", "var(--theme-state-success-bg)"),
    FamilyToken::new("callout-border-warning", "var(--theme-state-warning-bg)"),
    FamilyToken::new("callout-border-error", "var(--theme-state-error-bg)"),
    FamilyToken::new("callout-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("callout-title-font-size", "var(--font-size-sm)"),
    FamilyToken::new("callout-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("callout-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("callout-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("callout-description-line-height", "var(--line-height-normal)"),

    // Badge
    FamilyToken::new("badge-padding-x", "var(--space-sm)"),
    FamilyToken::new("badge-padding-y", "var(--space-xs)"),
    FamilyToken::new("badge-radius", "var(--radius-full)"),
    FamilyToken::new("badge-font-size", "var(--font-size-xs)"),
    FamilyToken::new("badge-font-weight", "var(--font-weight-medium)"),
    FamilyToken::new("badge-line-height", "var(--line-height-tight)"),
    FamilyToken::new("badge-bg-default", "var(--theme-action-secondary-bg)"),
    FamilyToken::new("badge-bg-primary", "var(--theme-action-primary-bg)"),
    FamilyToken::new("badge-bg-success", "var(--theme-state-success-bg)"),
    FamilyToken::new("badge-bg-warning", "var(--theme-state-warning-bg)"),
    FamilyToken::new("badge-bg-destructive", "var(--theme-state-error-bg)"),
    FamilyToken::new("badge-fg-default", "var(--theme-action-secondary-fg)"),
    FamilyToken::new("badge-fg-primary", "var(--theme-action-primary-fg)"),
    FamilyToken::new("badge-fg-success", "var(--theme-state-success-fg)"),
    FamilyToken::new("badge-fg-warning", "var(--theme-state-warning-fg)"),
    FamilyToken::new("badge-fg-destructive", "var(--theme-state-error-fg)"),
    FamilyToken::new("badge-bg-outline", "transparent"),
    FamilyToken::new("badge-fg-outline", "var(--theme-text-primary)"),
    FamilyToken::new("badge-border-outline", "var(--theme-border-primary)"),
    FamilyToken::new("badge-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("badge-transition-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("badge-hover-opacity", "0.8"),

    // Toast
    FamilyToken::new("toast-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("toast-padding", "var(--space-md)"),
    FamilyToken::new("toast-radius", "var(--radius-lg)"),
    FamilyToken::new("toast-shadow", "var(--shadow-lg)"),
    FamilyToken::new("toast-gap", "var(--space-sm)"),
    FamilyToken::new("toast-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("toast-border-width", "1px"),
    FamilyToken::new("toast-success-bg", "var(--theme-state-success-bg)"),
    FamilyToken::new("toast-success-border", "var(--theme-state-success-bg)"),
    FamilyToken::new("toast-warning-bg", "var(--theme-state-warning-bg)"),
    FamilyToken::new("toast-warning-border", "var(--theme-state-warning-bg)"),
    FamilyToken::new("toast-error-bg", "var(--theme-state-error-bg)"),
    FamilyToken::new("toast-error-border", "var(--theme-state-error-bg)"),
    FamilyToken::new("toast-z-index", "var(--layer-toast)"),
    FamilyToken::new("toast-viewport-padding", "var(--space-md)"),
    FamilyToken::new("toast-viewport-gap", "var(--space-sm)"),
    FamilyToken::new("toast-viewport-max-width", "var(--layout-width-lg)"),
    FamilyToken::new("toast-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("toast-title-font-size", "var(--font-size-sm)"),
    FamilyToken::new("toast-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("toast-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("toast-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("toast-close-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("toast-close-size", "var(--space-md)"),

    // Banner
    FamilyToken::new("banner-gap", "var(--space-sm)"),
    FamilyToken::new("banner-padding-x", "var(--space-md)"),
    FamilyToken::new("banner-padding-y", "var(--space-sm)"),
    FamilyToken::new("banner-radius", "0"),
    FamilyToken::new("banner-shadow", "var(--shadow-sm)"),
    FamilyToken::new("banner-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("banner-border-width", "0 0 1px 0"),
    FamilyToken::new("banner-bg-info", "var(--theme-state-info-bg)"),
    FamilyToken::new("banner-bg-success", "var(--theme-state-success-bg)"),
    FamilyToken::new("banner-bg-warning", "var(--theme-state-warning-bg)"),
    FamilyToken::new("banner-bg-error", "var(--theme-state-error-bg)"),
    FamilyToken::new("banner-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("banner-content-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("banner-content-font-size", "var(--font-size-sm)"),
    FamilyToken::new("banner-close-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("banner-close-fg-hover", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("banner-close-size", "var(--space-md)"),
    FamilyToken::new("banner-actions-gap", "var(--space-sm)"),

    // Inline Notice
    FamilyToken::new("inline-notice-gap", "var(--space-sm)"),
    FamilyToken::new("inline-notice-padding-x", "var(--space-sm)"),
    FamilyToken::new("inline-notice-padding-y", "var(--space-sm)"),
    FamilyToken::new("inline-notice-radius", "var(--radius-md)"),
    FamilyToken::new("inline-notice-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("inline-notice-border-width", "1px"),
    FamilyToken::new("inline-notice-bg-default", "var(--theme-surface-muted)"),
    FamilyToken::new("inline-notice-bg-info", "var(--theme-state-info-bg)"),
    FamilyToken::new("inline-notice-bg-success", "var(--theme-state-success-bg)"),
    FamilyToken::new("inline-notice-bg-warning", "var(--theme-state-warning-bg)"),
    FamilyToken::new("inline-notice-bg-error", "var(--theme-state-error-bg)"),
    FamilyToken::new("inline-notice-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("inline-notice-font-size", "var(--font-size-sm)"),
    FamilyToken::new("inline-notice-icon-size", "var(--space-md)"),

    // Empty State
    FamilyToken::new("empty-state-padding", "var(--space-xl)"),
    FamilyToken::new("empty-state-gap", "var(--space-md)"),
    FamilyToken::new("empty-state-icon-size", "var(--size-icon-lg)"),
    FamilyToken::new("empty-state-icon-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("empty-state-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("empty-state-title-font-size", "var(--font-size-lg)"),
    FamilyToken::new("empty-state-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("empty-state-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("empty-state-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("empty-state-actions-gap", "var(--space-sm)"),

    // Error State
    FamilyToken::new("error-state-padding", "var(--space-xl)"),
    FamilyToken::new("error-state-gap", "var(--space-md)"),
    FamilyToken::new("error-state-icon-size", "var(--size-icon-lg)"),
    FamilyToken::new("error-state-icon-fg", "var(--theme-state-error-bg)"),
    FamilyToken::new("error-state-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("error-state-title-font-size", "var(--font-size-lg)"),
    FamilyToken::new("error-state-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("error-state-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("error-state-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("error-state-actions-gap", "var(--space-sm)"),

    // Status Dot

    // Spinner
    FamilyToken::new("spinner-size", "var(--space-lg)"),
    FamilyToken::new("spinner-border-width", "2px"),
    FamilyToken::new("spinner-color", "var(--theme-action-primary-bg)"),
    FamilyToken::new("spinner-track-color", "var(--theme-surface-muted)"),
    FamilyToken::new("spinner-duration", "var(--motion-duration-slow)"),
    FamilyToken::new("spinner-ease", "var(--motion-ease-linear)"),

    // Progress
    FamilyToken::new("progress-height", "var(--space-sm)"),
    FamilyToken::new("progress-radius", "var(--radius-full)"),
    FamilyToken::new("progress-bg", "var(--theme-surface-muted)"),
    FamilyToken::new("progress-fill-bg", "var(--theme-action-primary-bg)"),
    FamilyToken::new("progress-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("progress-transition-ease", "var(--motion-ease-standard)"),

    // Skeleton
    FamilyToken::new("skeleton-bg", "var(--theme-surface-muted)"),
    FamilyToken::new("skeleton-radius", "var(--radius-md)"),
    FamilyToken::new("skeleton-pulse-duration", "var(--motion-duration-slow)"),
    FamilyToken::new("skeleton-pulse-ease", "var(--motion-ease-standard)"),
    // Alert (tokens simplificados faltantes)
    FamilyToken::new("alert-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("alert-gap", "var(--space-sm)"),
    FamilyToken::new("alert-padding", "var(--space-md)"),

    // Badge (tokens simplificados faltantes)
    FamilyToken::new("badge-bg", "var(--theme-action-secondary-bg)"),
    FamilyToken::new("badge-fg", "var(--theme-action-secondary-fg)"),

    // Callout (tokens simplificados faltantes)
    FamilyToken::new("callout-bg", "var(--theme-surface-muted)"),
    FamilyToken::new("callout-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("callout-padding", "var(--space-md)"),

    // Progress
    FamilyToken::new("progress-fg", "var(--theme-action-primary-fg)"),

    // Skeleton
    FamilyToken::new("skeleton-shimmer-duration", "var(--motion-duration-slow)"),

    // Spinner
    FamilyToken::new("spinner-size-small", "var(--space-md)"),
    FamilyToken::new("spinner-size-medium", "var(--space-lg)"),
    FamilyToken::new("spinner-size-large", "var(--space-xl)"),
    FamilyToken::new("spinner-rotation-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("spinner-rotation-easing", "var(--motion-ease-linear)"),
    FamilyToken::new("spinner-opacity-paused", "0.5"),


    // Pulse
    FamilyToken::new("pulse-size", "var(--space-sm)"),
    FamilyToken::new("pulse-delay", "calc(var(--motion-duration-slow) / 2)"),
    FamilyToken::new("pulse-size-small", "var(--space-xs)"),
    FamilyToken::new("pulse-size-large", "var(--space-md)"),
    FamilyToken::new("pulse-bg-subtle", "var(--theme-action-secondary-bg)"),
    FamilyToken::new("pulse-bg-emphasized", "var(--theme-action-primary-bg)"),

    // LoadingOverlay
    FamilyToken::new("loading-overlay-z-index", "var(--layer-overlay)"),
    FamilyToken::new("loading-overlay-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("loading-overlay-opacity", "0.8"),
    FamilyToken::new("loading-overlay-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("loading-overlay-transition-ease", "var(--motion-ease-standard)"),

    // Pulse - Motion Indicator with Visual Hierarchy
    FamilyToken::new("pulse-size", "var(--space-sm)"),
    FamilyToken::new("pulse-delay", "calc(var(--motion-duration-slow) / 2)"),

    // Pulse - Motion Indicator (Enterprise)
    FamilyToken::new("pulse-size", "var(--space-sm)"),
    FamilyToken::new("pulse-size-small", "var(--space-xs)"),
    FamilyToken::new("pulse-size-large", "var(--space-md)"),
    FamilyToken::new("pulse-radius", "var(--radius-full)"),
    
    // Variants - Base colors (no opacity modulation)
    FamilyToken::new("pulse-bg-subtle", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("pulse-bg-default", "var(--theme-action-primary-bg)"),
    FamilyToken::new("pulse-bg-emphasized", "var(--color-success)"),
    
    // Variants - Ripple scales
    FamilyToken::new("pulse-scale-subtle", "1.3"),
    FamilyToken::new("pulse-scale-default", "1.5"),
    FamilyToken::new("pulse-scale-emphasized", "1.8"),
    
    // Emphasized - Glow effect (pre-calculated)
    FamilyToken::new("pulse-shadow-emphasized", "0 0 8px 2px"),
    FamilyToken::new("pulse-shadow-color-emphasized", "hsl(142 71% 45% / 0.4)"),
    
    // Motion tokens (Family I)
    FamilyToken::new("pulse-duration-slow", "var(--motion-duration-deliberate)"),
    FamilyToken::new("pulse-duration-normal", "var(--motion-duration-slow)"),
    FamilyToken::new("pulse-duration-fast", "var(--motion-duration-fast)"),
    FamilyToken::new("pulse-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("pulse-delay", "calc(var(--motion-duration-slow) / 2)"),

    // EmptyTable - Empty state for tables
    FamilyToken::new("empty-table-padding", "var(--space-xl)"),
    FamilyToken::new("empty-table-content-gap", "var(--space-sm)"),
    FamilyToken::new("empty-table-title-font-size", "var(--font-size-lg)"),
    FamilyToken::new("empty-table-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("empty-table-title-fg", "var(--theme-text-primary)"),
    FamilyToken::new("empty-table-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("empty-table-description-line-height", "var(--line-height-relaxed)"),
    FamilyToken::new("empty-table-description-fg", "var(--theme-text-secondary)"),
];
