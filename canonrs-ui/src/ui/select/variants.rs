#[derive(Clone, Copy, Default)]
pub enum SelectSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SelectSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum SelectValidation {
    #[default]
    None,
    Error,
    Success,
    Warning,
}

impl SelectValidation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Error => "error",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}
