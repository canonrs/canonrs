use super::types::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Solid,
    Outline,
    Ghost,
    Danger,
    Warning,
    Success,
    Muted,
    Elevated,
    Inverted,
}

impl ButtonVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Solid => "bg-primary text-primary-foreground border-primary",
            Self::Outline => "bg-background text-foreground border-border",
            Self::Ghost => "bg-transparent text-muted-foreground border-transparent",
            Self::Danger => "bg-destructive text-destructive-foreground border-destructive",
            Self::Warning => "bg-warning text-foreground border-warning",
            Self::Success => "bg-success text-primary-foreground border-success",
            Self::Muted => "bg-muted text-muted-foreground border-muted",
            Self::Elevated => "bg-card text-card-foreground border-border shadow-md",
            Self::Inverted => "bg-foreground text-background border-foreground",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Icon,
}

impl ButtonSize {
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Xs => "px-2 text-xs",
            Self::Sm => "px-3 text-sm",
            Self::Md => "px-4 text-base",
            Self::Lg => "px-6 text-lg",
            Self::Xl => "px-8 text-xl",
            Self::Icon => "p-0",
        }
    }
    
    pub fn style(&self) -> &'static str {
        match self {
            Self::Xs => "height: var(--size-control-sm);",
            Self::Sm => "height: var(--size-control-sm);",
            Self::Md => "height: var(--size-control-md);",
            Self::Lg => "height: var(--size-control-lg);",
            Self::Xl => "height: var(--size-control-lg);",
            Self::Icon => "height: var(--size-control-md); width: var(--size-control-md);",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationState {
    Success,
    Warning,
    Error,
}

impl ValidationState {
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Success => "border-success",
            Self::Warning => "border-warning",
            Self::Error => "border-destructive",
        }
    }
}
