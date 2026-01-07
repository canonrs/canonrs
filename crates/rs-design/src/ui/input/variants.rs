#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl InputSize {
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Sm => "px-3 py-1 text-sm",
            Self::Md => "px-4 py-2 text-base",
            Self::Lg => "px-6 py-3 text-lg",
        }
    }
    
    pub fn style(&self) -> &'static str {
        match self {
            Self::Sm => "height: var(--size-control-sm);",
            Self::Md => "height: var(--size-control-md);",
            Self::Lg => "height: var(--size-control-lg);",
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
    pub fn border_classes(&self) -> &'static str {
        match self {
            Self::None => "border-border",
            Self::Success => "border-success",
            Self::Warning => "border-warning",
            Self::Error => "border-destructive",
        }
    }
    
    pub fn ring_classes(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Success => "focus-visible:ring-success",
            Self::Warning => "focus-visible:ring-warning",
            Self::Error => "focus-visible:ring-destructive",
        }
    }
}
