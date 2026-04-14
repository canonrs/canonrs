//! Component tokens — Blocks
//! Card, Hero, Section, StatGroup, PageHeader, FormField, DataTable, SidebarLayout

use crate::design::tokens::FamilyToken;

pub const BLOCKS_TOKENS: &[FamilyToken] = &[

    // ── Card Block ────────────────────────────────────────────────────────────
    FamilyToken::new("card-block-gap",             "var(--space-md)"),
    FamilyToken::new("card-block-padding",         "var(--space-lg)"),
    FamilyToken::new("card-block-bg",              "var(--theme-surface-bg)"),
    FamilyToken::new("card-block-radius",          "var(--radius-md)"),
    FamilyToken::new("card-block-shadow",          "var(--shadow-sm)"),
    FamilyToken::new("card-block-border-color",    "var(--theme-surface-border)"),
    FamilyToken::new("card-block-border-width",    "1px"),

    // ── Hero Block ────────────────────────────────────────────────────────────
    FamilyToken::new("hero-block-padding-x",       "var(--space-2xl)"),
    FamilyToken::new("hero-block-padding-y",       "var(--space-3xl)"),
    FamilyToken::new("hero-block-gap",             "var(--space-xl)"),
    FamilyToken::new("hero-block-media-gap",       "var(--space-2xl)"),
    FamilyToken::new("hero-block-actions-gap",     "var(--space-md)"),
    FamilyToken::new("hero-block-max-width",       "800px"),
    FamilyToken::new("hero-block-align",           "center"),
    FamilyToken::new("hero-block-min-height",      "80vh"),

    // ── Section Block ─────────────────────────────────────────────────────────
    FamilyToken::new("section-block-gap",              "var(--space-lg)"),
    FamilyToken::new("section-block-padding",          "var(--space-xl) 0"),
    FamilyToken::new("section-block-header-margin",    "0 0 var(--space-lg) 0"),
    FamilyToken::new("section-block-footer-padding",   "var(--space-md) 0 0 0"),
    FamilyToken::new("section-block-footer-border",    "1px solid var(--theme-surface-border)"),

    // ── StatGroup Block ───────────────────────────────────────────────────────
    FamilyToken::new("stat-group-block-gap",           "var(--space-md)"),
    FamilyToken::new("stat-group-block-padding",       "var(--space-lg) 0"),

    // ── PageHeader Block ──────────────────────────────────────────────────────
    FamilyToken::new("page-header-block-gap",              "var(--space-sm)"),
    FamilyToken::new("page-header-block-padding",          "var(--space-md) var(--space-lg)"),
    FamilyToken::new("page-header-block-breadcrumb-gap",   "var(--space-xs)"),
    FamilyToken::new("page-header-block-actions-gap",      "var(--space-sm)"),
    FamilyToken::new("page-header-block-border-color",     "var(--theme-surface-border)"),
    FamilyToken::new("page-header-block-border-width",     "1px"),

    // ── FormField Block ───────────────────────────────────────────────────────
    FamilyToken::new("form-field-block-gap",               "var(--space-xs)"),
    FamilyToken::new("form-field-block-label-gap",         "var(--space-xs)"),
    FamilyToken::new("form-field-block-hint-fg",           "var(--theme-surface-fg-muted)"),
    FamilyToken::new("form-field-block-hint-font-size",    "var(--font-size-sm)"),
    FamilyToken::new("form-field-block-error-fg",          "var(--color-destructive)"),
    FamilyToken::new("form-field-block-error-font-size",   "var(--font-size-sm)"),

    // ── DataTable Block ───────────────────────────────────────────────────────
    FamilyToken::new("data-table-block-gap",               "var(--space-md)"),
    FamilyToken::new("data-table-block-toolbar-padding",   "var(--space-sm) 0"),
    FamilyToken::new("data-table-block-toolbar-gap",       "var(--space-sm)"),
    FamilyToken::new("data-table-block-pagination-padding","var(--space-sm) 0"),
    FamilyToken::new("data-table-block-empty-padding",     "var(--space-2xl) 0"),

    // ── SidebarLayout Block ───────────────────────────────────────────────────
    FamilyToken::new("sidebar-layout-gap",             "var(--space-lg)"),
    FamilyToken::new("sidebar-layout-nav-width",       "var(--layout-sidebar-width)"),
    FamilyToken::new("sidebar-layout-nav-bg",          "var(--theme-surface-bg)"),
    FamilyToken::new("sidebar-layout-nav-border",      "var(--border-thin) solid var(--theme-surface-border)"),
    FamilyToken::new("sidebar-layout-nav-padding",     "var(--space-md)"),
    FamilyToken::new("sidebar-layout-main-padding",    "var(--space-lg)"),
];
