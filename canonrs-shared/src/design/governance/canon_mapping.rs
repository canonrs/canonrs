//! Canon Rule Compliance: Component → Token Family Mapping
//! 
//! This module enforces Canon Rules #21-28 by defining which token families
//! each component MUST use. Components cannot arbitrarily choose tokens.

use std::collections::HashMap;

/// Token families from Canon Rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenFamily {
    /// A) Overlay & Layering (Dialogs, Popovers, Menus)
    OverlayLayering,
    /// B) Selection & Lists (Tables, Checkboxes, Radios)
    SelectionLists,
    /// C) Forms & Validation (Inputs, Fields, Buttons)
    FormsValidation,
    /// D) Navigation & Structure (Tabs, Sidebar, Breadcrumbs)
    NavigationStructure,
    /// E) Feedback & Status (Alerts, Toasts, Progress)
    FeedbackStatus,
    /// F) Data & Media (Charts, Cards, Avatars)
    DataMedia,
}

/// Component classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentSpec {
    pub name: &'static str,
    pub families: &'static [TokenFamily],
    pub required_tokens: &'static [&'static str],
}

impl TokenFamily {
    /// Get required tokens for this family
    pub fn required_tokens(&self) -> &'static [&'static str] {
        use TokenFamily::*;
        match self {
            OverlayLayering => &[
                "z::modal", "z::popover", "z::dropdown",
                "shadows::lg", "shadows::xl",
                "motion::duration::normal", "motion::ease::emphasized",
            ],
            SelectionLists => &[
                "spacing::sm", "spacing::md",
                "state::focus_ring", "state::hover_opacity",
                "colors::primary", "colors::muted",
            ],
            FormsValidation => &[
                "spacing::field_height", "spacing::field_padding",
                "border::thin", "radius::md",
                "state::focus_ring", "state::disabled_opacity",
                "colors::destructive", "colors::success", "colors::warning",
            ],
            NavigationStructure => &[
                "spacing::md", "spacing::lg",
                "typography::font_size::sm", "typography::font_weight::medium",
                "colors::foreground", "colors::muted_foreground",
            ],
            FeedbackStatus => &[
                "motion::duration::normal",
                "colors::success", "colors::warning", "colors::destructive",
                "shadows::sm", "radius::md",
            ],
            DataMedia => &[
                "radius::md", "shadows::sm",
                "spacing::md", "spacing::lg",
                "colors::card", "colors::border",
            ],
        }
    }
}

/// Registry of all components and their token requirements
pub fn component_registry() -> HashMap<&'static str, ComponentSpec> {
    use TokenFamily::*;
    
    let mut registry = HashMap::new();
    
    // A) Overlay & Layering
    for component in ["Dialog", "Popover", "DropdownMenu", "HoverCard", "Sheet", "ContextMenu", "Menubar"] {
        registry.insert(component, ComponentSpec {
            name: component,
            families: &[OverlayLayering],
            required_tokens: OverlayLayering.required_tokens(),
        });
    }
    
    // B) Selection & Lists
    for component in ["Accordion", "Checkbox", "Command", "DataTable", "RadioGroup", "Table", "ToggleGroup"] {
        registry.insert(component, ComponentSpec {
            name: component,
            families: &[SelectionLists],
            required_tokens: SelectionLists.required_tokens(),
        });
    }
    registry
}
