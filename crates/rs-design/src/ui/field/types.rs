/// Field legend variant
#[derive(Clone, Copy, Default)]
pub enum FieldLegendVariant {
    #[default]
    Legend,
    Label,
}

impl FieldLegendVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Legend => "legend",
            Self::Label => "label",
        }
    }
}
