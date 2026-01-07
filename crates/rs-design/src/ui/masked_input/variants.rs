use super::types::MaskedInputVariant;

impl MaskedInputVariant {
    pub fn border_color(&self) -> &'static str {
        match self {
            MaskedInputVariant::Default => "border-border", // color.border.default
            MaskedInputVariant::Success => "border-success-border", // color.success.border
            MaskedInputVariant::Error => "border-danger-border", // color.danger.border
            MaskedInputVariant::Warning => "border-warning-border", // color.warning.border
        }
    }

    pub fn focus_ring(&self) -> &'static str {
        match self {
            MaskedInputVariant::Default => "focus:ring-2 focus:ring-primary-bg", // state.focus.ring
            MaskedInputVariant::Success => "focus:ring-2 focus:ring-success-border",
            MaskedInputVariant::Error => "focus:ring-2 focus:ring-danger-border",
            MaskedInputVariant::Warning => "focus:ring-2 focus:ring-warning-border",
        }
    }
}
