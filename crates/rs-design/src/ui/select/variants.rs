/// Select size variants (Família C - field.height, field.padding)
#[derive(Clone, Copy, Default)]
pub enum SelectSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl SelectSize {
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Sm => "\
                h-[var(--field-height)] \
                px-[var(--space-sm)] \
                py-[var(--field-padding)] \
                text-[length:var(--font-size-sm)]",
            Self::Md => "\
                h-[var(--field-height)] \
                px-[var(--space-md)] \
                py-[var(--field-padding)] \
                text-[length:var(--font-size-md)]",
            Self::Lg => "\
                h-[var(--field-height)] \
                px-[var(--space-lg)] \
                py-[var(--field-padding)] \
                text-[length:var(--font-size-lg)]",
        }
    }
}

/// Select validation state (Família C - validation.*)
#[derive(Clone, Copy, Default)]
pub enum SelectValidation {
    #[default]
    None,
    Error,
    Success,
    Warning,
}

impl SelectValidation {
    pub fn border_classes(&self) -> &'static str {
        match self {
            Self::None => "border-[var(--field-border)]",
            Self::Error => "border-danger",
            Self::Success => "border-success",
            Self::Warning => "border-warning",
        }
    }
    
    pub fn ring_classes(&self) -> &'static str {
        match self {
            Self::None => "focus:ring-[color:var(--state-focus-ring)]",
            Self::Error => "focus:ring-[hsl(var(--validation-error))]",
            Self::Success => "focus:ring-[hsl(var(--validation-success))]",
            Self::Warning => "focus:ring-[hsl(var(--validation-warning))]",
        }
    }
}
