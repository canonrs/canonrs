/// Combobox size variants (Família C - field.height)
#[derive(Clone, Copy, Default)]
pub enum ComboboxSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ComboboxSize {
    pub fn trigger_classes(&self) -> &'static str {
        match self {
            Self::Sm => "\
                h-[var(--field-height)] \
                px-[var(--space-sm)] \
                text-[length:var(--font-size-sm)]",
            Self::Md => "\
                h-[var(--field-height)] \
                px-[var(--space-md)] \
                text-[length:var(--font-size-md)]",
            Self::Lg => "\
                h-[var(--field-height)] \
                px-[var(--space-lg)] \
                text-[length:var(--font-size-lg)]",
        }
    }
    
    pub fn list_item_classes(&self) -> &'static str {
        match self {
            Self::Sm => "h-[var(--list-item-height)] py-[var(--space-xs)]",
            Self::Md => "h-[var(--list-item-height)] py-[var(--space-sm)]",
            Self::Lg => "h-[var(--list-item-height)] py-[var(--space-md)]",
        }
    }
}

/// Combobox validation state (Família C)
#[derive(Clone, Copy, Default)]
pub enum ComboboxValidation {
    #[default]
    None,
    Error,
    Success,
    Warning,
}

impl ComboboxValidation {
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
