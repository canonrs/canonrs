use crate::design::tokens::FamilyToken;

/// FAMILY D — Navigation & Structure
/// Components: NavigationMenu, Pagination, TableOfContents, Tabs, Breadcrumb, Link, Toolbar
/// Scope: Navigation patterns, content organization (excludes: Sidebar - see Family H Layout)

pub const FAMILY_D_NAVIGATION: &[FamilyToken] = &[
    // Navigation foundation
    FamilyToken::new("navigation-color", "var(--semantic-brand-solid)"),
    FamilyToken::new("navigation-spacing", "var(--space-sm)"),
    FamilyToken::new("navigation-font-weight", "var(--font-weight-medium)"),

    // Navigation Menu
    FamilyToken::new("navigation-menu-bg", "var(--semantic-surface-base)"),
    FamilyToken::new("navigation-menu-padding", "var(--space-sm)"),
    FamilyToken::new("navigation-menu-gap", "var(--space-xs)"),
    FamilyToken::new("navigation-menu-border-color", "var(--semantic-surface-border)"),
    FamilyToken::new("navigation-menu-border-width", "1px"),
    FamilyToken::new("navigation-menu-trigger-fg", "var(--semantic-text-primary)"),
    FamilyToken::new("navigation-menu-trigger-bg-hover", "var(--semantic-surface-hover)"),
    FamilyToken::new("navigation-menu-trigger-bg-expanded", "var(--semantic-surface-hover)"),
    FamilyToken::new("navigation-menu-trigger-icon-size", "var(--space-md)"),
    FamilyToken::new("navigation-menu-trigger-icon-rotation", "180deg"),
    FamilyToken::new("navigation-menu-trigger-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("navigation-menu-trigger-transition-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("navigation-menu-item-fg", "var(--semantic-text-primary)"),
    FamilyToken::new("navigation-menu-item-height", "var(--size-nav-item)"),
    FamilyToken::new("navigation-menu-item-padding-x", "var(--space-md)"),
    FamilyToken::new("navigation-menu-item-padding-y", "var(--space-sm)"),
    FamilyToken::new("navigation-menu-item-bg-hover", "var(--semantic-surface-hover)"),
    FamilyToken::new("navigation-menu-item-bg-active", "var(--semantic-surface-hover)"),
    FamilyToken::new("navigation-menu-link-fg", "var(--semantic-text-primary)"),
    FamilyToken::new("navigation-menu-link-fg-hover", "var(--semantic-brand-solid)"),
    FamilyToken::new("navigation-menu-link-text-decoration", "none"),
    FamilyToken::new("navigation-menu-content-bg", "var(--semantic-surface-elevated)"),
    FamilyToken::new("navigation-menu-content-padding", "var(--space-md)"),
    FamilyToken::new("navigation-menu-content-radius", "var(--radius-md)"),
    FamilyToken::new("navigation-menu-content-shadow", "var(--shadow-lg)"),
    FamilyToken::new("navigation-menu-content-border-color", "var(--semantic-surface-border)"),
    FamilyToken::new("navigation-menu-content-border-width", "1px"),

    // Pagination
    FamilyToken::new("pagination-gap", "var(--space-sm)"),
    FamilyToken::new("pagination-item-size", "var(--size-nav-item)"),
    FamilyToken::new("pagination-item-padding", "var(--space-sm)"),
    FamilyToken::new("pagination-item-radius", "var(--radius-sm)"),
    FamilyToken::new("pagination-item-bg", "transparent"),
    FamilyToken::new("pagination-item-bg-hover", "var(--semantic-surface-hover)"),
    FamilyToken::new("pagination-item-bg-active", "var(--semantic-brand-solid)"),
    FamilyToken::new("pagination-item-fg", "var(--semantic-text-primary)"),
    FamilyToken::new("pagination-item-fg-active", "var(--color-primary-foreground)"),
    FamilyToken::new("pagination-item-font-size", "var(--font-size-sm)"),
    FamilyToken::new("pagination-item-font-weight", "var(--font-weight-normal)"),
    FamilyToken::new("pagination-item-font-weight-active", "var(--font-weight-medium)"),
    FamilyToken::new("pagination-item-disabled-opacity", "var(--state-disabled-opacity)"),
    FamilyToken::new("pagination-item-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("pagination-item-transition-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("pagination-ellipsis-fg", "var(--semantic-text-tertiary)"),

    // Table of Contents
    FamilyToken::new("toc-width", "var(--layout-width-toc)"),
    FamilyToken::new("toc-bg", "transparent"),
    FamilyToken::new("toc-gap", "var(--space-sm)"),
    FamilyToken::new("toc-title-fg", "var(--semantic-text-primary)"),
    FamilyToken::new("toc-title-font-size", "var(--font-size-sm)"),
    FamilyToken::new("toc-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("toc-item-fg", "var(--semantic-text-secondary)"),
    FamilyToken::new("toc-item-fg-hover", "var(--semantic-text-primary)"),
    FamilyToken::new("toc-item-active-fg", "var(--semantic-brand-solid)"),
    FamilyToken::new("toc-item-font-size", "var(--font-size-sm)"),
    FamilyToken::new("toc-item-indent-step", "var(--space-md)"),
    FamilyToken::new("toc-button-padding-x", "var(--space-sm)"),
    FamilyToken::new("toc-button-padding-y", "var(--space-sm)"),
    FamilyToken::new("toc-button-radius", "var(--radius-sm)"),

    // Tabs
    FamilyToken::new("tabs-gap", "var(--space-sm)"),
    FamilyToken::new("tabs-border-color", "var(--semantic-surface-border)"),
    FamilyToken::new("tabs-border-width", "1px"),
    FamilyToken::new("tab-fg", "var(--semantic-text-secondary)"),
    FamilyToken::new("tab-fg-hover", "var(--semantic-text-primary)"),
    FamilyToken::new("tab-fg-active", "var(--semantic-text-primary)"),
    FamilyToken::new("tab-padding-x", "var(--space-md)"),
    FamilyToken::new("tab-padding-y", "var(--space-sm)"),
    FamilyToken::new("tab-font-size", "var(--font-size-sm)"),
    FamilyToken::new("tab-font-weight", "var(--font-weight-normal)"),
    FamilyToken::new("tab-font-weight-active", "var(--font-weight-medium)"),
    FamilyToken::new("tab-indicator-color", "var(--semantic-brand-solid)"),
    FamilyToken::new("tab-indicator-height", "2px"),
    FamilyToken::new("tab-content-padding-y", "var(--space-md)"),
    FamilyToken::new("tab-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("tab-transition-ease", "var(--motion-ease-standard)"),

    // Breadcrumb (has interactive version)
    FamilyToken::new("breadcrumb-fg", "var(--semantic-text-secondary)"),
    FamilyToken::new("breadcrumb-fg-hover", "var(--semantic-text-primary)"),
    FamilyToken::new("breadcrumb-fg-active", "var(--semantic-text-primary)"),
    FamilyToken::new("breadcrumb-font-size", "var(--font-size-sm)"),
    FamilyToken::new("breadcrumb-font-weight", "var(--font-weight-normal)"),
    FamilyToken::new("breadcrumb-font-weight-active", "var(--font-weight-medium)"),
    FamilyToken::new("breadcrumb-gap", "var(--space-sm)"),
    FamilyToken::new("breadcrumb-separator-fg", "var(--semantic-text-tertiary)"),
    FamilyToken::new("breadcrumb-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("breadcrumb-transition-ease", "var(--motion-ease-standard)"),

    // Link
    FamilyToken::new("link-fg", "var(--semantic-brand-solid)"),
    FamilyToken::new("link-fg-hover", "var(--semantic-brand-hover)"),
    FamilyToken::new("link-fg-active", "var(--semantic-brand-active)"),
    FamilyToken::new("link-font-weight", "var(--font-weight-normal)"),
    FamilyToken::new("link-text-decoration", "underline"),
    FamilyToken::new("link-text-decoration-hover", "none"),
    FamilyToken::new("link-disabled-opacity", "var(--state-disabled-opacity)"),
    FamilyToken::new("link-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("link-transition-ease", "var(--motion-ease-standard)"),

    // Toolbar
    FamilyToken::new("toolbar-bg", "var(--semantic-surface-base)"),
    FamilyToken::new("toolbar-border", "1px solid var(--semantic-surface-border)"),
    FamilyToken::new("toolbar-gap", "var(--space-sm)"),
    FamilyToken::new("toolbar-padding", "var(--space-sm)"),
    // Nav Item (has interactive version)
    FamilyToken::new("nav-item-bg", "transparent"),
    FamilyToken::new("nav-item-hover-bg", "var(--color-accent)"),
    FamilyToken::new("nav-item-padding", "var(--space-sm) var(--space-md)"),

];
