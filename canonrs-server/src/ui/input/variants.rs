#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl InputSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputValidation {
    #[default]
    None,
    Success,
    Warning,
    Error,
}

impl InputValidation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}
