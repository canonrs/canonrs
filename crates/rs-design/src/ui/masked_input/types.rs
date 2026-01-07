use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MaskType {
    CPF,
    CNPJ,
    Phone,
    CEP,
    Date,
    CreditCard,
    Custom(String),
}

impl MaskType {
    pub fn pattern(&self) -> &str {
        match self {
            MaskType::CPF => "999.999.999-99",
            MaskType::CNPJ => "99.999.999/9999-99",
            MaskType::Phone => "(99) 99999-9999",
            MaskType::CEP => "99999-999",
            MaskType::Date => "99/99/9999",
            MaskType::CreditCard => "9999 9999 9999 9999",
            MaskType::Custom(pattern) => pattern,
        }
    }

    pub fn max_length(&self) -> usize {
        match self {
            MaskType::CPF => 14,
            MaskType::CNPJ => 18,
            MaskType::Phone => 15,
            MaskType::CEP => 9,
            MaskType::Date => 10,
            MaskType::CreditCard => 19,
            MaskType::Custom(pattern) => pattern.len(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MaskedInputVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
}
