use crate::design::tokens::FamilyToken;

/// FAMILY H — Layout & Composition
/// Components: PageHeader, PageLayout, Sidebar, Section, Separator, BlockHeader, BlockFooter, Footer, Header
/// Scope: Page structure, layout regions, compositional hierarchy

pub const FAMILY_H_LAYOUT: &[FamilyToken] = &[
    // Layout foundation
    FamilyToken::new("layout-gap", "var(--space-md)"),
    FamilyToken::new("layout-padding", "var(--space-md)"),
    FamilyToken::new("layout-max-width", "var(--layout-content-max-width)"),
    FamilyToken::new("layout-min-height", "100vh"),
    FamilyToken::new("layout-header-height", "var(--layout-height-header)"),
    FamilyToken::new("layout-footer-height", "var(--layout-height-header)"),
    FamilyToken::new("layout-sidebar-width", "var(--layout-sidebar-width)"),
    FamilyToken::new("layout-panel-width", "var(--layout-width-md)"),
    FamilyToken::new("layout-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("layout-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("layout-border-width", "1px"),
    FamilyToken::new("layout-scrollbar-size", "var(--space-sm)"),
    FamilyToken::new("layout-scrollbar-bg", "var(--theme-surface-muted)"),
    FamilyToken::new("layout-transition-duration", "var(--motion-duration-normal)"),
    FamilyToken::new("layout-transition-ease", "var(--motion-ease-standard)"),

    // Page Header
    FamilyToken::new("page-header-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("page-header-padding", "var(--space-xl) var(--space-md)"),
    FamilyToken::new("page-header-padding-bottom", "var(--space-lg)"),
    FamilyToken::new("page-header-margin-bottom", "var(--space-xl)"),
    FamilyToken::new("page-header-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("page-header-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("page-header-border-width", "0 0 1px 0"),
    FamilyToken::new("page-header-gap", "var(--space-md)"),
    FamilyToken::new("page-header-content-gap", "var(--space-sm)"),
    FamilyToken::new("page-header-actions-gap", "var(--space-sm)"),
    FamilyToken::new("page-header-title-fg", "var(--theme-surface-fg)"),
    FamilyToken::new("page-header-title-font-size", "var(--font-size-3xl)"),
    FamilyToken::new("page-header-title-font-weight", "var(--font-weight-bold)"),
    FamilyToken::new("page-header-title-line-height", "var(--line-height-tight)"),
    FamilyToken::new("page-header-subtitle-spacing", "var(--space-sm)"),
    FamilyToken::new("page-header-description-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("page-header-description-font-size", "var(--font-size-sm)"),
    FamilyToken::new("page-header-description-line-height", "var(--line-height-normal)"),
    FamilyToken::new("page-header-breadcrumbs-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("page-header-breadcrumbs-gap", "var(--space-sm)"),
    FamilyToken::new("page-header-breadcrumb-spacing", "var(--space-sm)"),
    FamilyToken::new("page-header-tabs-margin-top", "var(--space-md)"),

    // Page Layout
    FamilyToken::new("page-layout-gap", "var(--space-lg)"),
    FamilyToken::new("page-layout-content-padding", "var(--space-lg)"),
    FamilyToken::new("page-layout-sidebar-width", "var(--layout-sidebar-width)"),
    FamilyToken::new("page-layout-sidebar-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("page-layout-sidebar-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("page-layout-sidebar-padding", "var(--space-md)"),
    FamilyToken::new("page-layout-aside-width", "var(--layout-width-md)"),
    FamilyToken::new("page-layout-aside-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("page-layout-aside-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("page-layout-aside-padding", "var(--space-md)"),

    // Sidebar
    FamilyToken::new("sidebar-width", "var(--layout-sidebar-width)"),
    FamilyToken::new("sidebar-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("sidebar-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("sidebar-border-color", "var(--theme-surface-border)"),
    FamilyToken::new("sidebar-border-width", "1px"),
    FamilyToken::new("sidebar-padding", "var(--space-md)"),
    FamilyToken::new("sidebar-content-padding", "var(--space-md)"),
    FamilyToken::new("sidebar-inset-padding-left", "var(--space-md)"),
    FamilyToken::new("sidebar-header-padding", "var(--space-md)"),
    FamilyToken::new("sidebar-header-margin", "0 0 var(--space-md) 0"),
    FamilyToken::new("sidebar-header-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("sidebar-footer-padding", "var(--space-md)"),
    FamilyToken::new("sidebar-footer-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("sidebar-menu-gap", "var(--space-xs)"),
    FamilyToken::new("sidebar-menu-item-height", "var(--space-xl)"),
    FamilyToken::new("sidebar-menu-item-padding-x", "var(--space-sm)"),
    FamilyToken::new("sidebar-menu-item-padding-y", "var(--space-sm)"),
    FamilyToken::new("sidebar-menu-item-gap", "var(--space-sm)"),
    FamilyToken::new("sidebar-menu-item-radius", "var(--radius-sm)"),
    FamilyToken::new("sidebar-menu-item-font-size", "var(--font-size-sm)"),
    FamilyToken::new("sidebar-group-label-fg", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("sidebar-group-label-font-size", "var(--font-size-xs)"),
    FamilyToken::new("sidebar-group-label-font-weight", "var(--font-weight-semibold)"),
    FamilyToken::new("sidebar-group-label-padding-x", "var(--space-sm)"),
    FamilyToken::new("sidebar-group-label-padding-y", "var(--space-sm)"),
    FamilyToken::new("sidebar-separator-color", "var(--theme-surface-border)"),
    FamilyToken::new("sidebar-separator-height", "1px"),
    FamilyToken::new("sidebar-separator-margin-y", "var(--space-sm)"),

    // Section
    FamilyToken::new("section-padding", "var(--space-xl) 0"),
    FamilyToken::new("section-header-margin", "0 0 var(--space-lg) 0"),
    FamilyToken::new("section-title-color", "var(--theme-surface-fg)"),
    FamilyToken::new("section-title-size", "var(--font-size-2xl)"),
    FamilyToken::new("section-title-weight", "var(--font-weight-bold)"),
    FamilyToken::new("section-description-color", "var(--theme-surface-fg-muted)"),
    FamilyToken::new("section-description-size", "var(--font-size-sm)"),
    FamilyToken::new("section-description-margin", "var(--space-sm) 0 0 0"),
    FamilyToken::new("section-card-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("section-card-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("section-card-radius", "var(--radius-md)"),
    FamilyToken::new("section-card-shadow", "var(--shadow-sm)"),

    // Separator
    FamilyToken::new("separator-color", "var(--theme-surface-border)"),
    FamilyToken::new("separator-color-muted", "var(--theme-surface-muted)"),
    FamilyToken::new("separator-thickness", "1px"),
    FamilyToken::new("separator-length-horizontal", "100%"),
    FamilyToken::new("separator-length-vertical", "var(--space-lg)"),
    FamilyToken::new("separator-margin-x", "0"),
    FamilyToken::new("separator-margin-y", "var(--space-md)"),

    // Block Header
    FamilyToken::new("block-header-gap", "var(--space-sm)"),

    // Block Footer
    FamilyToken::new("block-footer-gap", "var(--space-sm)"),
    FamilyToken::new("block-footer-padding", "var(--space-md) 0 0 0"),
    FamilyToken::new("block-footer-border", "1px solid var(--theme-surface-border)"),

    // Layout Regions
    FamilyToken::new("layout-region-gap", "var(--space-lg)"),
    FamilyToken::new("layout-region-padding", "var(--space-lg)"),

    // Footer
    FamilyToken::new("footer-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("footer-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("footer-padding", "var(--space-lg)"),
    FamilyToken::new("footer-start-gap", "var(--space-md)"),
    FamilyToken::new("footer-end-gap", "var(--space-md)"),

    // Header
    FamilyToken::new("header-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("header-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("header-height", "var(--layout-height-header)"),
    FamilyToken::new("header-padding", "0 var(--space-lg)"),
    FamilyToken::new("header-start-gap", "var(--space-md)"),
    FamilyToken::new("header-end-gap", "var(--space-md)"),
];
