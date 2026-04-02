use crate::design::tokens::FamilyToken;

// Components: Layout — PageHeader, PageLayout, Sidebar, Section, Separator, Header, Footer
/// Components: PageHeader, PageLayout, Sidebar, Section, Separator, BlockHeader, BlockFooter, Footer, Header
/// Scope: Page structure, layout regions, compositional hierarchy

pub const LAYOUT_TOKENS: &[FamilyToken] = &[
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
    FamilyToken::new("page-header-padding-bottom", "var(--space-lg)"),
    FamilyToken::new("page-header-margin-bottom", "var(--space-xl)"),
    FamilyToken::new("page-header-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("page-header-content-gap", "var(--space-sm)"),
    FamilyToken::new("page-header-title-line-height", "var(--line-height-tight)"),
    FamilyToken::new("page-header-subtitle-spacing", "var(--space-sm)"),
    FamilyToken::new("page-header-description-line-height", "var(--line-height-normal)"),
    FamilyToken::new("page-header-breadcrumb-spacing", "var(--space-sm)"),

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
    FamilyToken::new("sidebar-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("sidebar-header-margin", "0 0 var(--space-md) 0"),
    FamilyToken::new("sidebar-header-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("sidebar-footer-border", "1px solid var(--theme-surface-border)"),

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
    FamilyToken::new("footer-gap", "var(--space-lg)"),

    // Hero Block
    FamilyToken::new("hero-padding-x", "var(--space-2xl)"),
    FamilyToken::new("hero-padding-y", "var(--space-3xl)"),
    FamilyToken::new("hero-gap", "var(--space-xl)"),
    FamilyToken::new("hero-media-gap", "var(--space-2xl)"),
    FamilyToken::new("hero-actions-gap", "var(--space-md)"),
    FamilyToken::new("hero-max-width", "800px"),
    FamilyToken::new("hero-align", "center"),
    FamilyToken::new("hero-min-height", "80vh"),

    // Header
    FamilyToken::new("header-bg", "var(--theme-surface-bg)"),
    FamilyToken::new("layout-divider-color", "rgba(0,0,0,0.08)"),
    FamilyToken::new("layout-header-border-color", "var(--layout-divider-color)"),
    FamilyToken::new("header-border", "1px solid var(--layout-header-border-color)"),
    FamilyToken::new("header-height", "var(--layout-height-header)"),
    FamilyToken::new("header-padding", "0 var(--space-lg)"),
    FamilyToken::new("header-start-gap", "var(--space-md)"),
    FamilyToken::new("header-end-gap", "var(--space-md)"),

    // Typography Global
    FamilyToken::new("font-family-sans",  "var(--primitive-font-sans)"),
    FamilyToken::new("font-family-serif", "var(--primitive-font-serif)"),
    FamilyToken::new("font-family-mono",  "var(--primitive-font-mono)"),

    // Logo
    // Logo
    FamilyToken::new("logo-icon-size-sm", "1.5rem"),
    FamilyToken::new("logo-icon-size-md", "2rem"),
    FamilyToken::new("logo-icon-size-lg", "2.5rem"),
    FamilyToken::new("logo-wordmark-font-size-sm", "var(--font-size-sm)"),
    FamilyToken::new("logo-wordmark-font-size-md", "var(--primitive-font-size-5)"),
    FamilyToken::new("logo-wordmark-font-size-lg", "var(--primitive-font-size-6)"),
    FamilyToken::new("logo-wordmark-font-weight", "var(--font-weight-bold)"),
    FamilyToken::new("logo-wordmark-color", "var(--theme-action-primary-bg)"),
    FamilyToken::new("logo-gap", "var(--space-sm)"),
    FamilyToken::new("logo-letter-spacing", "-0.02em"),
    // Layout dimension primitives — valores reais
    FamilyToken::new("layout-height-header", "56px"),
    FamilyToken::new("layout-height-footer", "56px"),
    FamilyToken::new("layout-sidebar-width", "240px"),
    FamilyToken::new("layout-sidebar-width-collapsed", "60px"),
    FamilyToken::new("layout-content-max-width", "1280px"),
    FamilyToken::new("layout-width-xs", "200px"),
    FamilyToken::new("layout-width-sm", "280px"),
    FamilyToken::new("layout-width-md", "380px"),
    FamilyToken::new("layout-width-lg", "560px"),
    FamilyToken::new("layout-width-xl", "720px"),
    FamilyToken::new("layout-width-dialog", "480px"),
    FamilyToken::new("layout-width-toc", "220px"),
    FamilyToken::new("layout-aside-width", "280px"),

    // Builder Mode — usa tokens do tema como fonte da verdade
    FamilyToken::new("builder-region-border",        "var(--theme-action-primary-bg)"),
    FamilyToken::new("builder-region-border-hover",  "color-mix(in srgb, var(--theme-action-primary-bg) 60%, transparent)"),
    FamilyToken::new("builder-region-bg-hover",      "color-mix(in srgb, var(--theme-action-primary-bg) 5%, transparent)"),
    FamilyToken::new("builder-region-bg-active",     "color-mix(in srgb, var(--theme-action-primary-bg) 20%, transparent)"),
    FamilyToken::new("builder-region-bg-pulse",      "color-mix(in srgb, var(--theme-action-primary-bg) 18%, transparent)"),
    FamilyToken::new("builder-region-label-color",   "color-mix(in srgb, var(--theme-action-primary-bg) 70%, transparent)"),
    FamilyToken::new("builder-region-label-bg",      "color-mix(in srgb, var(--theme-action-primary-bg) 12%, transparent)"),
    FamilyToken::new("builder-region-meta-color",    "color-mix(in srgb, var(--theme-action-primary-bg) 50%, transparent)"),
    FamilyToken::new("builder-insert-line-color",    "var(--theme-action-primary-bg)"),
    FamilyToken::new("builder-placeholder-color",    "color-mix(in srgb, var(--theme-action-primary-bg) 50%, transparent)"),
    FamilyToken::new("builder-transition-duration",  "var(--motion-duration-fast)"),
    FamilyToken::new("builder-transition-ease",      "var(--motion-ease-standard)"),
    FamilyToken::new("builder-pulse-duration",       "var(--motion-duration-slow)"),

];
// ─── Layout Dimension Primitives (valores reais) ─────────────────────────────
