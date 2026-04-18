use crate::design::tokens::FamilyToken;

// Components: Navigation — Sidebar, NavigationMenu, Pagination, TOC, Tabs, Breadcrumb, Link, Toolbar
/// Components: NavigationMenu, Pagination, TableOfContents, Tabs, Breadcrumb, Link, Toolbar, Menubar, Sidebar
/// Scope: Navigation patterns, content organization

pub const NAVIGATION_TOKENS: &[FamilyToken] = &[
    // Navigation foundation
    FamilyToken::new("navigation-color", "var(--theme-action-primary-bg)"),
    FamilyToken::new("navigation-spacing", "var(--space-sm)"),
    FamilyToken::new("navigation-font-weight", "var(--font-weight-medium)"),

    // Menubar
    FamilyToken::new("menubar-gap", "var(--space-xs)"),
    FamilyToken::new("menubar-padding", "var(--space-xs)"),
    FamilyToken::new("menubar-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("menubar-radius", "var(--radius-md)"),
    FamilyToken::new("menubar-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("menubar-border-width", "1px"),

    // Sidebar
    FamilyToken::new("sidebar-width", "16rem"),
    FamilyToken::new("sidebar-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("sidebar-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("sidebar-border-width", "1px"),
    FamilyToken::new("sidebar-padding", "var(--space-md)"),
    FamilyToken::new("sidebar-menu-gap", "var(--space-xs)"),
    FamilyToken::new("sidebar-menu-item-height", "var(--space-2xl)"),
    FamilyToken::new("sidebar-menu-item-padding-x", "var(--space-sm)"),
    FamilyToken::new("sidebar-menu-item-padding-y", "var(--space-sm)"),
    FamilyToken::new("sidebar-menu-item-radius", "var(--radius-sm)"),
    FamilyToken::new("sidebar-menu-item-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("sidebar-menu-item-font-size", "var(--font-size-sm)"),
    FamilyToken::new("sidebar-header-padding", "var(--space-md)"),
    FamilyToken::new("sidebar-content-padding", "var(--space-md)"),
    FamilyToken::new("sidebar-footer-padding", "var(--space-md)"),
    FamilyToken::new("sidebar-menu-item-gap", "var(--space-sm)"),
    FamilyToken::new("sidebar-menu-item-bg", "transparent"),
    FamilyToken::new("sidebar-menu-item-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("sidebar-menu-item-transition-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("sidebar-menu-item-font-weight-active", "var(--font-weight-medium)"),
    FamilyToken::new("sidebar-menu-item-bg-hover", "var(--theme-surface-muted)"),
    FamilyToken::new("sidebar-menu-item-bg-active", "var(--theme-action-accent-bg)"),
    FamilyToken::new("sidebar-menu-item-fg-active", "var(--theme-action-primary-fg)"),
    FamilyToken::new("sidebar-group-label-padding-x", "var(--space-sm)"),
    FamilyToken::new("sidebar-group-label-padding-y", "var(--space-xs)"),
    FamilyToken::new("sidebar-group-label-font-size", "var(--font-size-xs)"),
    FamilyToken::new("sidebar-group-label-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("sidebar-group-label-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("sidebar-separator-height", "var(--border-thin)"),
    FamilyToken::new("sidebar-separator-color", "var(--theme-surface-border)"),
    FamilyToken::new("sidebar-separator-margin-y", "var(--space-sm)"),
    FamilyToken::new("sidebar-inset-padding-left",    "var(--space-lg)"),
    FamilyToken::new("sidebar-width-collapsed",        "var(--layout-sidebar-width-collapsed)"),

    // Navigation Menu
    FamilyToken::new("navigation-menu-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("navigation-menu-padding", "var(--space-sm)"),
    FamilyToken::new("navigation-menu-gap", "var(--space-xs)"),
    FamilyToken::new("navigation-menu-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("navigation-menu-border-width", "1px"),
    FamilyToken::new("navigation-menu-trigger-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("navigation-menu-trigger-icon-size", "var(--space-md)"),
    FamilyToken::new("navigation-menu-trigger-icon-rotation", "180deg"),
    FamilyToken::new("navigation-menu-trigger-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("navigation-menu-trigger-transition-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("navigation-menu-item-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("navigation-menu-item-height", "var(--size-nav-item)"),
    FamilyToken::new("navigation-menu-item-padding-x", "var(--space-md)"),
    FamilyToken::new("navigation-menu-item-padding-y", "var(--space-sm)"),
    FamilyToken::new("navigation-menu-link-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("navigation-menu-link-text-decoration", "none"),
    FamilyToken::new("navigation-menu-content-bg", "var(--theme-overlay-bg)"),
    FamilyToken::new("navigation-menu-content-padding", "var(--space-md)"),
    FamilyToken::new("navigation-menu-content-radius", "var(--radius-md)"),
    FamilyToken::new("navigation-menu-content-shadow", "var(--shadow-lg)"),
    FamilyToken::new("navigation-menu-trigger-bg-hover", "var(--theme-surface-muted)"),
    FamilyToken::new("navigation-menu-trigger-bg-active", "var(--theme-action-accent-bg)"),
    FamilyToken::new("navigation-menu-trigger-fg-active", "var(--theme-action-primary-fg)"),
    FamilyToken::new("navigation-menu-link-fg-hover", "var(--theme-action-primary-bg)"),
    FamilyToken::new("navigation-menu-content-border-width", "1px"),
    FamilyToken::new("navigation-menu-content-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("navigation-menu-z-index", "var(--layer-overlay)"),
    FamilyToken::new("navigation-menu-content-min-width", "var(--layout-width-sm)"),
    FamilyToken::new("navigation-menu-list-gap", "var(--space-xs)"),
    FamilyToken::new("navigation-menu-trigger-hover-fg", "var(--nav-item-hover-fg)"),
    FamilyToken::new("navigation-menu-trigger-active-fg", "var(--nav-item-active-fg)"),

    // Pagination
    FamilyToken::new("pagination-gap", "var(--space-sm)"),
    FamilyToken::new("pagination-item-size", "var(--size-nav-item)"),
    FamilyToken::new("pagination-item-padding", "var(--space-sm)"),
    FamilyToken::new("pagination-item-radius", "var(--radius-sm)"),
    FamilyToken::new("pagination-item-bg", "transparent"),
    FamilyToken::new("pagination-item-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("pagination-item-font-size", "var(--font-size-sm)"),
    FamilyToken::new("pagination-item-font-weight", "var(--font-weight-normal)"),
    FamilyToken::new("pagination-item-font-weight-active", "var(--font-weight-medium)"),
    FamilyToken::new("pagination-item-disabled-opacity", "var(--opacity-disabled)"),
    FamilyToken::new("pagination-item-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("pagination-item-transition-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("pagination-ellipsis-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("pagination-item-bg-hover", "var(--theme-surface-muted)"),
    FamilyToken::new("pagination-item-bg-active", "var(--theme-action-primary-bg)"),
    FamilyToken::new("pagination-item-fg-active", "var(--color-primary-foreground)"),

    // ── Table of Contents ─────────────────────────────────────────────────────
    FamilyToken::new("toc-width",                    "var(--layout-width-toc)"),
    FamilyToken::new("toc-bg",                       "transparent"),
    FamilyToken::new("toc-gap",                      "var(--space-xs)"),

    // TOC Title
    FamilyToken::new("toc-title-fg",                 "var(--theme-surface-fg-muted)"),
    FamilyToken::new("toc-title-font-size",          "var(--font-size-xs)"),
    FamilyToken::new("toc-title-font-weight",        "var(--font-weight-bold)"),
    FamilyToken::new("toc-title-letter-spacing",     "0.06em"),

    // TOC Link
    FamilyToken::new("toc-link-fg",                  "var(--theme-surface-fg-muted)"),
    FamilyToken::new("toc-link-font-size",           "var(--font-size-sm)"),
    FamilyToken::new("toc-link-padding-x",           "var(--space-sm)"),
    FamilyToken::new("toc-link-padding-y",           "var(--space-xs)"),
    FamilyToken::new("toc-link-radius",              "var(--radius-sm)"),
    FamilyToken::new("toc-link-border-width",        "2px"),

    // TOC Active State
    FamilyToken::new("toc-link-border-active",       "var(--theme-action-primary-bg)"),
    FamilyToken::new("toc-link-font-weight-active",  "var(--font-weight-semibold)"),

    // TOC Indentation
    FamilyToken::new("toc-indent-l3",                "var(--space-md)"),
    FamilyToken::new("toc-indent-l4",                "var(--space-lg)"),
    FamilyToken::new("toc-indent-l5",                "var(--space-xl)"),

    // TOC Expand Button (mode: nested)
    FamilyToken::new("toc-expand-btn-size",          "var(--space-md)"),
    FamilyToken::new("toc-expand-btn-fg",            "var(--theme-surface-fg-muted)"),
    FamilyToken::new("toc-expand-btn-radius",        "var(--radius-sm)"),

    // TOC Subtree (mode: nested)
    FamilyToken::new("toc-subtree-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("toc-subtree-transition-ease",     "var(--motion-ease-standard)"),

    // TOC Child visibility (mode: expand)
    FamilyToken::new("toc-child-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("toc-child-transition-ease",     "var(--motion-ease-standard)"),

    // TOC Sticky Sidebar
    FamilyToken::new("toc-sticky-top",               "var(--space-xl)"),
    FamilyToken::new("toc-sticky-padding",           "var(--space-md)"),
    FamilyToken::new("toc-sticky-border-color",      "var(--theme-surface-border-muted)"),
    FamilyToken::new("toc-sticky-border-radius",     "var(--radius-md)"),
    FamilyToken::new("toc-sticky-bg",                "var(--theme-surface-bg)"),

    // TOC legacy (kept for compat)
    FamilyToken::new("toc-item-fg",                  "var(--theme-surface-fg-muted)"),
    FamilyToken::new("toc-item-active-fg",           "var(--theme-action-primary-bg)"),
    FamilyToken::new("toc-item-font-size",           "var(--font-size-sm)"),
    FamilyToken::new("toc-item-indent-step",         "var(--space-md)"),
    FamilyToken::new("toc-button-padding-x",         "var(--space-sm)"),
    FamilyToken::new("toc-button-padding-y",         "var(--space-sm)"),
    FamilyToken::new("toc-button-radius",            "var(--radius-sm)"),

    // TOC ancestor state
    FamilyToken::new("toc-link-fg-ancestor",          "var(--theme-surface-fg)"),
    FamilyToken::new("toc-link-bg-ancestor",          "transparent"),
    FamilyToken::new("toc-link-border-ancestor",      "color-mix(in srgb, var(--theme-action-primary-bg) 40%, transparent)"),
    FamilyToken::new("toc-link-font-weight-ancestor", "var(--font-weight-medium)"),

    // TOC expand button states

    // TOC subtree animation
    FamilyToken::new("toc-subtree-gap",               "var(--space-2xs)"),
    FamilyToken::new("toc-child-max-height",          "2.5rem"),
    FamilyToken::new("toc-subtree-indent",            "var(--space-sm)"),

    // TOC level indentation
    FamilyToken::new("toc-indent-l2",                 "var(--space-sm)"),
    FamilyToken::new("toc-link-bg-hover",             "var(--theme-surface-muted)"),
    FamilyToken::new("toc-progress-scale",            "0"),

    // Tabs
    FamilyToken::new("tabs-gap", "var(--space-sm)"),
    FamilyToken::new("tabs-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("tabs-border-width", "1px"),
    FamilyToken::new("tab-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("tab-padding-x", "var(--space-md)"),
    FamilyToken::new("tab-padding-y", "var(--space-sm)"),
    FamilyToken::new("tab-font-size", "var(--font-size-sm)"),
    FamilyToken::new("tab-font-weight", "var(--font-weight-normal)"),
    FamilyToken::new("tab-font-weight-active", "var(--font-weight-medium)"),
    FamilyToken::new("tab-indicator-color", "var(--theme-action-primary-bg)"),
    FamilyToken::new("tab-indicator-height", "2px"),
    FamilyToken::new("tab-content-padding-y", "var(--space-md)"),
    FamilyToken::new("tab-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("tab-transition-ease", "var(--motion-ease-standard)"),

    // Breadcrumb
    FamilyToken::new("breadcrumb-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("breadcrumb-font-size", "var(--font-size-sm)"),
    FamilyToken::new("breadcrumb-font-weight", "var(--font-weight-normal)"),
    FamilyToken::new("breadcrumb-font-weight-active", "var(--font-weight-medium)"),
    FamilyToken::new("breadcrumb-gap", "var(--space-sm)"),
    FamilyToken::new("breadcrumb-separator-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("breadcrumb-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("breadcrumb-transition-ease", "var(--motion-ease-standard)"),
    FamilyToken::new("breadcrumb-collapse-min-width", "12rem"),
    FamilyToken::new("breadcrumb-collapse-max-width", "20rem"),
    FamilyToken::new("breadcrumb-fg-hover", "var(--theme-action-primary-bg)"),
    FamilyToken::new("breadcrumb-page-fg", "var(--theme-action-primary-fg)"),
    FamilyToken::new("breadcrumb-collapse-border-width", "1px"),
    FamilyToken::new("link-group-label-letter-spacing", "var(--letter-spacing-wide)"),

    // Link
    FamilyToken::new("link-fg", "var(--theme-action-primary-bg)"),
    FamilyToken::new("link-font-weight", "var(--font-weight-normal)"),
    FamilyToken::new("link-text-decoration", "none"),
    FamilyToken::new("link-text-decoration-hover", "underline"),
    FamilyToken::new("link-disabled-opacity", "var(--opacity-disabled)"),
    FamilyToken::new("link-transition-duration", "var(--motion-duration-fast)"),
    FamilyToken::new("link-fg-hover",            "color-mix(in srgb, var(--theme-action-primary-bg) 80%, white)"),
    FamilyToken::new("link-fg-active",           "var(--theme-action-primary-active)"),
    FamilyToken::new("link-transition-ease",     "var(--motion-ease-standard)"),
    FamilyToken::new("link-muted-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("link-underline-decoration", "underline"),
    FamilyToken::new("link-default-decoration", "none"),
    FamilyToken::new("link-default-decoration-hover", "underline"),
    FamilyToken::new("link-muted-decoration", "none"),
    FamilyToken::new("link-muted-decoration-hover", "none"),
    FamilyToken::new("link-underline-decoration-hover", "underline"),

    // Toolbar
    FamilyToken::new("toolbar-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("toolbar-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("toolbar-gap", "var(--space-sm)"),
    FamilyToken::new("toolbar-padding", "var(--space-sm)"),
    FamilyToken::new("toolbar-item-padding-x",       "var(--space-sm)"),
    FamilyToken::new("toolbar-item-padding-y",       "var(--space-xs)"),
    FamilyToken::new("toolbar-item-radius",          "var(--radius-sm)"),
    FamilyToken::new("toolbar-item-fg",              "var(--theme-surface-fg)"),
    FamilyToken::new("toolbar-item-bg-hover",        "var(--theme-surface-muted)"),
    FamilyToken::new("toolbar-item-fg-hover",        "var(--theme-surface-fg)"),
    FamilyToken::new("toolbar-item-bg-pressed",      "var(--theme-action-primary-bg)"),
    FamilyToken::new("toolbar-item-fg-pressed",      "var(--theme-action-primary-fg)"),
    FamilyToken::new("toolbar-item-size",            "var(--size-nav-item)"),
    FamilyToken::new("toolbar-item-transition",      "var(--motion-duration-fast)"),
    FamilyToken::new("toolbar-item-disabled-opacity","var(--opacity-disabled)"),
    // Breadcrumb (tokens simplificados faltantes)
    FamilyToken::new("breadcrumb-item-fg", "var(--theme-surface-fg-muted)"),

    // Menu (tokens simplificados faltantes)
    FamilyToken::new("menu-item-bg", "transparent"),
    FamilyToken::new("menu-item-padding", "var(--space-sm)"),

    // NavItem
    FamilyToken::new("nav-item-height", "var(--space-2xl)"),
    FamilyToken::new("nav-item-padding-x", "var(--space-sm)"),
    FamilyToken::new("nav-item-padding-y", "var(--space-xs)"),
    FamilyToken::new("nav-item-gap", "var(--space-xs)"),
    FamilyToken::new("nav-item-radius", "var(--radius-sm)"),
    FamilyToken::new("nav-item-font-size", "var(--font-size-sm)"),
    FamilyToken::new("nav-item-font-weight", "var(--font-weight-medium)"),
    FamilyToken::new("nav-item-active-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("nav-item-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("nav-item-icon-size", "1.25em"),
    FamilyToken::new("nav-item-line-height", "var(--line-height-normal)"),
    FamilyToken::new("nav-item-hover-bg", "transparent"),
    FamilyToken::new("nav-item-hover-fg", "var(--theme-action-primary-bg)"),
    FamilyToken::new("nav-item-active-bg", "transparent"),
    FamilyToken::new("nav-item-active-fg", "var(--theme-action-primary-bg)"),
    FamilyToken::new("nav-item-focus-ring-width", "var(--focus-ring-width)"),
    FamilyToken::new("nav-item-focus-ring-color", "var(--theme-action-primary-bg)"),

    // Link Group
    FamilyToken::new("link-group-gap", "var(--space-xs)"),
    FamilyToken::new("link-group-label-font-size", "var(--font-size-xs)"),
    FamilyToken::new("link-group-label-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("link-group-label-color", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("link-group-label-margin-bottom", "var(--space-xs)"),
    FamilyToken::new("link-group-horizontal-gap",      "var(--space-md)"),
    FamilyToken::new("link-group-item-hover-bg",        "var(--theme-surface-muted)"),
    FamilyToken::new("link-group-item-hover-fg",        "var(--theme-surface-fg)"),
    FamilyToken::new("link-group-item-active-fg",       "var(--theme-action-primary-bg)"),
    FamilyToken::new("link-group-item-active-border",   "var(--theme-action-primary-bg)"),
    FamilyToken::new("link-group-item-active-weight",   "var(--font-weight-semibold)"),
    FamilyToken::new("link-group-item-padding-x",       "var(--space-sm)"),
    FamilyToken::new("link-group-item-padding-y",       "var(--space-xs)"),
    FamilyToken::new("link-group-item-radius",          "var(--radius-sm)"),
    FamilyToken::new("link-group-item-border-width",    "var(--border-medium)"),


        // PageHeader
    FamilyToken::new("page-header-gap", "var(--space-sm)"),
    FamilyToken::new("page-header-padding", "var(--space-md) var(--space-lg)"),
    FamilyToken::new("page-header-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("page-header-border-width", "1px"),
    FamilyToken::new("page-header-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("page-header-breadcrumbs-gap", "var(--space-xs)"),
    FamilyToken::new("page-header-breadcrumbs-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("page-header-title-font-size", "var(--font-size-xl)"),
    FamilyToken::new("page-header-title-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("page-header-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("page-header-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("page-header-description-line-height", "var(--line-height-normal)"),
    FamilyToken::new("page-header-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("page-header-actions-gap", "var(--space-sm)"),
    FamilyToken::new("page-header-tabs-margin-top", "var(--space-sm)"),

        // ── DocProgress (Reading progress indicator) ─────────────────────────────
    FamilyToken::new("doc-progress-height",     "3px"),
    FamilyToken::new("doc-progress-bg",         "color-mix(in srgb, var(--theme-surface-fg) 15%, transparent)"),
    FamilyToken::new("doc-progress-bar-bg",     "var(--theme-action-primary-bg)"),
    FamilyToken::new("doc-progress-z-index",    "1000"),
    FamilyToken::new("doc-progress-transition", "width var(--motion-duration-normal) var(--motion-ease-standard)"),
];