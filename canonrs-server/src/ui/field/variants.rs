#[derive(Clone, Copy, Default)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

impl FieldOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Responsive => "responsive",
        }
    }
}

#[derive(Clone, Copy, Default)]
pub enum FieldValidation {
    #[default]
    None,
    Error,
    Success,
    Warning,
}

impl FieldValidation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Error => "error",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}
