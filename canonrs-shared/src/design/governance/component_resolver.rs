//! Semantic resolver: UI folder → Canonical Component
//! This is the CONTRACT layer between filesystem and token families

/// Resolves UI directory name to canonical component name in token registry
pub fn resolve_canonical_component(ui: &str) -> &'static str {
    match ui {
        // A) Overlay → Dialog family
        "alert_dialog" | "confirm_dialog" | "drawer" | "modal" => "Dialog",
        
        // C) Forms → canonical forms
        "icon_button" => "Button",
        "input_group" => "Input",
        "input_otp" => "InputOTP",
        "color_picker" => "ColorPicker",
        
        // E) Feedback → FeedbackStatus
        "empty_state" | "error_state" | "empty_table" => "EmptyState",
        "form_error_summary" => "FormErrorSummary",
        "inline_notice" => "InlineNotice",
        "loading_overlay" => "LoadingOverlay",
        
        // D) Navigation → canonical nav
        "nav_item" => "NavigationMenu",
        "page_header" => "PageHeader",
        
        // F) Data → canonical data
        "code_block" => "CodeBlock",
        "status_dot" => "StatusDot",
        "table_of_contents" => "TableOfContents",
        
        // Default: assume PascalCase matches
        _ => {
            // For components where folder name = canonical name (snake→pascal)
            // Examples: accordion, alert, avatar, badge, button, card, etc.
            snake_to_pascal_static(ui)
        }
    }
}

// Helper: convert snake_case to PascalCase (for simple cases)
fn snake_to_pascal_static(s: &str) -> &'static str {
    // This is a fallback. In production, use a proper converter or macro
    // For now, registry keys must match exactly or be in the match above
    panic!("Component '{}' needs explicit resolver mapping", s)
}
