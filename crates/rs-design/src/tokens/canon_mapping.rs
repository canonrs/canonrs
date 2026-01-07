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
    
    // C) Forms & Validation
    for component in ["Button", "Field", "Form", "Input", "InputOTP", "NativeSelect", "Slider", "Switch", "Textarea", "Toggle"] {
        registry.insert(component, ComponentSpec {
            name: component,
            families: &[FormsValidation],
            required_tokens: FormsValidation.required_tokens(),
        });
    }
    
    // D) Navigation & Structure
    for component in ["Breadcrumb", "ButtonGroup", "Kbd", "NavigationMenu", "Pagination", "Sidebar", "Tabs", "Typography"] {
        registry.insert(component, ComponentSpec {
            name: component,
            families: &[NavigationStructure],
            required_tokens: NavigationStructure.required_tokens(),
        });
    }
    
    // E) Feedback & Status
    for component in ["Alert", "Badge", "Progress", "Skeleton", "Spinner", "Toast", "Sonner"] {
        registry.insert(component, ComponentSpec {
            name: component,
            families: &[FeedbackStatus],
            required_tokens: FeedbackStatus.required_tokens(),
        });
    }
    
    // F) Data & Media
    for component in ["Avatar", "Calendar", "Card", "Carousel", "Chart"] {
        registry.insert(component, ComponentSpec {
            name: component,
            families: &[DataMedia],
            required_tokens: DataMedia.required_tokens(),
        });
    }
    
    // Multi-family components
    registry.insert("Select", ComponentSpec {
        name: "Select",
        families: &[FormsValidation, SelectionLists, OverlayLayering],
        required_tokens: &[
            "spacing::field_height",
            "state::focus_ring",
            "z::dropdown",
            "spacing::sm",
        ],
    });
    
    registry.insert("Combobox", ComponentSpec {
        name: "Combobox",
        families: &[FormsValidation, SelectionLists, OverlayLayering],
        required_tokens: &[
            "spacing::field_height",
            "state::focus_ring",
            "z::popover",
            "spacing::sm",
        ],
    });
    
    registry.insert("DatePicker", ComponentSpec {
        name: "DatePicker",
        families: &[FormsValidation, DataMedia, OverlayLayering],
        required_tokens: &[
            "spacing::field_height",
            "z::popover",
            "radius::md",
        ],
    });
    
    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_components_registered() {
        let registry = component_registry();
        assert!(registry.contains_key("Button"));
        assert!(registry.contains_key("Select"));
        assert!(registry.contains_key("Dialog"));
    }
    
    #[test]
    fn test_multi_family_components() {
        let registry = component_registry();
        let select = registry.get("Select").unwrap();
        assert_eq!(select.families.len(), 3);
    }
}
