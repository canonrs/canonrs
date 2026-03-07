// Semantic Color System
pub struct SemanticColors {
    pub background: &'static str,
    pub foreground: &'static str,
    pub muted: &'static str,
    pub muted_foreground: &'static str,
    pub accent: &'static str,
    pub accent_foreground: &'static str,
    pub border: &'static str,
    pub ring: &'static str,
    pub error: &'static str,
}

pub const SEMANTIC: SemanticColors = SemanticColors {
    background: "hsl(var(--color-background))",
    foreground: "hsl(var(--color-foreground))",
    muted: "hsl(var(--color-muted))",
    muted_foreground: "hsl(var(--color-muted-foreground))",
    accent: "hsl(var(--color-accent))",
    accent_foreground: "hsl(var(--color-accent-foreground))",
    border: "hsl(var(--color-border))",
    ring: "hsl(var(--color-ring))",
    error: "hsl(var(--color-destructive))",
};

// Associated constant for backward compatibility
impl SemanticColors {
    pub const ERROR: &'static str = "hsl(var(--color-destructive))";
}
