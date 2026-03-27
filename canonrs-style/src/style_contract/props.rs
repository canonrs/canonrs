//! StyleProps — unified contract + class resolver

use super::{
    spacing::Spacing,
    layout_style::LayoutStyle,
    typography::Typography,
    color::ColorStyle,
};

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StyleProps {
    pub spacing:    Option<Spacing>,
    pub layout:     Option<LayoutStyle>,
    pub typography: Option<Typography>,
    pub color:      Option<ColorStyle>,
}

impl StyleProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_class(&self) -> String {
        let mut classes: Vec<&str> = Vec::new();

        if let Some(spacing) = &self.spacing {
            if let Some(m) = &spacing.margin {
                classes.push(m.as_margin_class());
            }
            if let Some(p) = &spacing.padding {
                classes.push(p.as_padding_class());
            }
        }

        if let Some(layout) = &self.layout {
            if let Some(align) = &layout.align {
                classes.push(align.as_class());
            }
            if let Some(width) = &layout.width {
                classes.push(width.as_class());
            }
        }

        if let Some(typography) = &self.typography {
            if let Some(size) = &typography.size {
                classes.push(size.as_class());
            }
            if let Some(weight) = &typography.weight {
                classes.push(weight.as_class());
            }
            if let Some(align) = &typography.align {
                classes.push(align.as_class());
            }
        }

        if let Some(color) = &self.color {
            let c = color.as_class();
            if !c.is_empty() {
                classes.push(c);
            }
        }

        classes.into_iter().filter(|c| !c.is_empty()).collect::<Vec<_>>().join(" ")
    }

    /// Returns classes prefixed with breakpoint, e.g. "md:m-md md:p-sm"
    pub fn to_class_bp(&self, bp: &str) -> String {
        let base = self.to_class();
        if base.is_empty() {
            return String::new();
        }
        base.split_whitespace()
            .map(|c| format!("{}:{}", bp, c))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn merge_with_class(&self, base_class: &str) -> String {
        let style_class = self.to_class();
        if style_class.is_empty() {
            base_class.to_string()
        } else if base_class.is_empty() {
            style_class
        } else {
            format!("{} {}", base_class, style_class)
        }
    }

    pub fn merge(&self, other: &StyleProps) -> StyleProps {
        StyleProps {
            spacing:    other.spacing.clone().or_else(|| self.spacing.clone()),
            layout:     other.layout.clone().or_else(|| self.layout.clone()),
            typography: other.typography.clone().or_else(|| self.typography.clone()),
            color:      other.color.clone().or_else(|| self.color.clone()),
        }
    }
}
