//! Color/variant contract

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Variant {
    #[default] Default,
    Primary, Secondary, Success, Warning, Danger, Ghost, Outline,
}

impl Variant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default   => "default",
            Self::Primary   => "primary",
            Self::Secondary => "secondary",
            Self::Success   => "success",
            Self::Warning   => "warning",
            Self::Danger    => "danger",
            Self::Ghost     => "ghost",
            Self::Outline   => "outline",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "default"   => Some(Self::Default),
            "primary"   => Some(Self::Primary),
            "secondary" => Some(Self::Secondary),
            "success"   => Some(Self::Success),
            "warning"   => Some(Self::Warning),
            "danger"    => Some(Self::Danger),
            "ghost"     => Some(Self::Ghost),
            "outline"   => Some(Self::Outline),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ColorStyle {
    pub variant: Option<Variant>,
}
