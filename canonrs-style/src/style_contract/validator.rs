//! StyleContract validator — ensures only valid token values

use super::spacing::SpaceScale;
use super::layout_style::{Align, Width};
use super::typography::{TextSize, TextWeight, TextAlign};
use super::color::Variant;
use super::props::StyleProps;

#[derive(Debug, Clone)]
pub struct StyleValidationError {
    pub field: &'static str,
    pub message: String,
}

impl std::fmt::Display for StyleValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StyleValidation[{}]: {}", self.field, self.message)
    }
}

/// Validate StyleProps — returns list of violations
pub fn validate_style(props: &StyleProps) -> Vec<StyleValidationError> {
    let errors = vec![];
    // All enums are valid by construction — validation is for JSON deserialization
    // Future: add custom constraints (e.g. max padding, forbidden combinations)
    let _ = props;
    errors
}

/// Parse style from JSON value — returns StyleProps or errors
pub fn style_from_json(value: &serde_json::Value) -> Result<StyleProps, Vec<StyleValidationError>> {
    use super::spacing::Spacing;
    use super::layout_style::LayoutStyle;
    use super::typography::Typography;
    use super::color::ColorStyle;

    let mut style = StyleProps::default();
    let mut errors = vec![];

    if let Some(spacing) = value.get("spacing") {
        let mut s = Spacing::default();
        if let Some(m) = spacing.get("margin").and_then(|v| v.as_str()) {
            match SpaceScale::from_str(m) {
                Some(scale) => s.margin = Some(scale),
                None => errors.push(StyleValidationError { field: "spacing.margin", message: format!("unknown value: {}", m) }),
            }
        }
        if let Some(p) = spacing.get("padding").and_then(|v| v.as_str()) {
            match SpaceScale::from_str(p) {
                Some(scale) => s.padding = Some(scale),
                None => errors.push(StyleValidationError { field: "spacing.padding", message: format!("unknown value: {}", p) }),
            }
        }
        style.spacing = Some(s);
    }

    if let Some(layout) = value.get("layout") {
        let mut l = LayoutStyle::default();
        if let Some(a) = layout.get("align").and_then(|v| v.as_str()) {
            match Align::from_str(a) {
                Some(align) => l.align = Some(align),
                None => errors.push(StyleValidationError { field: "layout.align", message: format!("unknown value: {}", a) }),
            }
        }
        if let Some(w) = layout.get("width").and_then(|v| v.as_str()) {
            match Width::from_str(w) {
                Some(width) => l.width = Some(width),
                None => errors.push(StyleValidationError { field: "layout.width", message: format!("unknown value: {}", w) }),
            }
        }
        style.layout = Some(l);
    }

    if let Some(typography) = value.get("typography") {
        let mut t = Typography::default();
        if let Some(s) = typography.get("size").and_then(|v| v.as_str()) {
            match TextSize::from_str(s) {
                Some(size) => t.size = Some(size),
                None => errors.push(StyleValidationError { field: "typography.size", message: format!("unknown value: {}", s) }),
            }
        }
        if let Some(w) = typography.get("weight").and_then(|v| v.as_str()) {
            match TextWeight::from_str(w) {
                Some(weight) => t.weight = Some(weight),
                None => errors.push(StyleValidationError { field: "typography.weight", message: format!("unknown value: {}", w) }),
            }
        }
        if let Some(a) = typography.get("align").and_then(|v| v.as_str()) {
            match TextAlign::from_str(a) {
                Some(align) => t.align = Some(align),
                None => errors.push(StyleValidationError { field: "typography.align", message: format!("unknown value: {}", a) }),
            }
        }
        style.typography = Some(t);
    }

    if let Some(color) = value.get("color") {
        let mut c = ColorStyle::default();
        if let Some(v) = color.get("variant").and_then(|v| v.as_str()) {
            match Variant::from_str(v) {
                Some(variant) => c.variant = Some(variant),
                None => errors.push(StyleValidationError { field: "color.variant", message: format!("unknown value: {}", v) }),
            }
        }
        style.color = Some(c);
    }

    if errors.is_empty() { Ok(style) } else { Err(errors) }
}
