use crate::design::tokens::FamilyToken;

// Components: Blocks — Grid, Columns, Container, Stack, Text, Markdown, Sidebar Layout
/// Components: Grid, Columns, Container, Stack, Text (h1-h3, p, caption, label), Markdown
/// Scope: Compositional content blocks used inside layout regions

pub const BLOCKS_TOKENS: &[FamilyToken] = &[
    // Grid Block
    FamilyToken::new("grid-gap",             "var(--space-md)"),
    FamilyToken::new("grid-min-col-width",   "200px"),
    FamilyToken::new("grid-padding",         "var(--space-md)"),
    FamilyToken::new("grid-bg",              "transparent"),
    FamilyToken::new("grid-border-color",    "var(--theme-surface-border)"),
    FamilyToken::new("grid-radius",          "var(--radius-md)"),

    // Columns Block
    FamilyToken::new("columns-gap",          "var(--space-md)"),
    FamilyToken::new("columns-padding",      "var(--space-md)"),
    FamilyToken::new("columns-bg",           "transparent"),
    FamilyToken::new("columns-radius",       "var(--radius-md)"),

    // Container Block
    FamilyToken::new("container-padding",    "var(--space-lg)"),
    FamilyToken::new("container-max-width",  "var(--layout-content-max-width)"),
    FamilyToken::new("container-bg",         "transparent"),
    FamilyToken::new("container-radius",     "var(--radius-md)"),

    // Stack Block
    FamilyToken::new("stack-gap",            "var(--space-md)"),
    FamilyToken::new("stack-padding",        "0"),
    FamilyToken::new("stack-bg",             "transparent"),

    // Text — Headings
    FamilyToken::new("text-h1-size",         "var(--font-size-4xl)"),
    FamilyToken::new("text-h1-weight",       "var(--font-weight-bold)"),
    FamilyToken::new("text-h1-color",        "var(--theme-surface-fg)"),
    FamilyToken::new("text-h1-line-height",  "var(--line-height-tight)"),
    FamilyToken::new("text-h2-size",         "var(--font-size-3xl)"),
    FamilyToken::new("text-h2-weight",       "var(--font-weight-semibold)"),
    FamilyToken::new("text-h2-color",        "var(--theme-surface-fg)"),
    FamilyToken::new("text-h2-line-height",  "var(--line-height-tight)"),
    FamilyToken::new("text-h3-size",         "var(--font-size-2xl)"),
    FamilyToken::new("text-h3-weight",       "var(--font-weight-semibold)"),
    FamilyToken::new("text-h3-color",        "var(--theme-surface-fg)"),
    FamilyToken::new("text-h3-line-height",  "var(--line-height-tight)"),

    // Text — Body
    FamilyToken::new("text-p-size",          "var(--font-size-base)"),
    FamilyToken::new("text-p-color",         "var(--theme-surface-fg)"),
    FamilyToken::new("text-p-line-height",   "var(--line-height-normal)"),
    FamilyToken::new("text-caption-size",    "var(--font-size-xs)"),
    FamilyToken::new("text-caption-color",   "var(--theme-surface-fg-muted)"),
    FamilyToken::new("text-caption-line-height", "var(--line-height-normal)"),
    FamilyToken::new("text-label-size",      "var(--font-size-sm)"),
    FamilyToken::new("text-label-weight",    "var(--font-weight-medium)"),
    FamilyToken::new("text-label-color",     "var(--theme-surface-fg)"),

    // Markdown Block
    FamilyToken::new("markdown-line-height", "var(--line-height-relaxed)"),
    FamilyToken::new("markdown-color",       "var(--theme-surface-fg)"),
    FamilyToken::new("markdown-gap",         "var(--space-md)"),
    FamilyToken::new("markdown-code-bg",     "var(--theme-surface-muted)"),
    FamilyToken::new("markdown-code-radius", "var(--radius-sm)"),

    // Sidebar Layout Block
    FamilyToken::new("sidebar-layout-nav-width",    "240px"),
    FamilyToken::new("sidebar-layout-nav-bg",       "var(--theme-surface-muted)"),
    FamilyToken::new("sidebar-layout-nav-border",   "1px solid var(--theme-surface-border)"),
    FamilyToken::new("sidebar-layout-main-padding", "var(--space-lg)"),

    // Split Block
    FamilyToken::new("split-gap",        "var(--space-md)"),
    FamilyToken::new("split-padding",    "0"),
    FamilyToken::new("split-bg",         "transparent"),

    // Detail Panel Block
    FamilyToken::new("detail-panel-aside-width",  "320px"),
    FamilyToken::new("detail-panel-aside-border", "1px solid var(--theme-surface-border)"),
    FamilyToken::new("detail-panel-padding",      "var(--space-md)"),

    // Timeline Block
    FamilyToken::new("timeline-gap",          "var(--space-lg)"),
    FamilyToken::new("timeline-line-color",   "var(--theme-surface-border)"),
    FamilyToken::new("timeline-line-width",   "2px"),
    FamilyToken::new("timeline-dot-size",     "12px"),
    FamilyToken::new("timeline-dot-color",    "var(--theme-primary-bg)"),

    // Wizard Block
    FamilyToken::new("wizard-steps-bg",      "var(--theme-surface-muted)"),
    FamilyToken::new("wizard-steps-padding", "var(--space-md)"),
    FamilyToken::new("wizard-body-padding",  "var(--space-lg)"),
    FamilyToken::new("wizard-actions-gap",   "var(--space-sm)"),

    // Filter Bar Block
    FamilyToken::new("filter-bar-gap",        "var(--space-sm)"),
    FamilyToken::new("filter-bar-padding",    "var(--space-sm) var(--space-md)"),
    FamilyToken::new("filter-bar-bg",         "var(--theme-surface-muted)"),
    FamilyToken::new("filter-bar-border",     "1px solid var(--theme-surface-border)"),
    FamilyToken::new("filter-bar-radius",     "var(--radius-md)"),
];
