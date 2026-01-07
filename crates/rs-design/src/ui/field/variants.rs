/// Field orientation variants
#[derive(Clone, Copy, Default)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

impl FieldOrientation {
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Vertical => "\
                flex-col \
                [&>*]:w-full \
                [&>.sr-only]:w-auto",
            Self::Horizontal => "\
                flex-row \
                items-center \
                [&>[data-slot=field-label]]:flex-auto \
                has-[>[data-slot=field-content]]:items-start \
                has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
            Self::Responsive => "\
                flex-col \
                [&>*]:w-full \
                [&>.sr-only]:w-auto \
                @md/field-group:flex-row \
                @md/field-group:items-center \
                @md/field-group:[&>*]:w-auto \
                @md/field-group:[&>[data-slot=field-label]]:flex-auto \
                @md/field-group:has-[>[data-slot=field-content]]:items-start \
                @md/field-group:has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px",
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Responsive => "responsive",
        }
    }
}

/// Field validation state (Família C - validation tokens)
#[derive(Clone, Copy, Default)]
pub enum FieldValidation {
    #[default]
    None,
    Error,
    Success,
    Warning,
}

impl FieldValidation {
    pub fn border_classes(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Error => "border-danger",
            Self::Success => "border-success",
            Self::Warning => "border-warning",
        }
    }
    
    pub fn text_classes(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Error => "text-danger-foreground",
            Self::Success => "text-success-foreground",
            Self::Warning => "text-warning-foreground",
        }
    }
}
