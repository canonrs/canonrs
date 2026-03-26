//! Color/variant contract

#[derive(Clone, Copy, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
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
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Default   => "",
            Self::Primary   => "variant-primary",
            Self::Secondary => "variant-secondary",
            Self::Success   => "variant-success",
            Self::Warning   => "variant-warning",
            Self::Danger    => "variant-danger",
            Self::Ghost     => "variant-ghost",
            Self::Outline   => "variant-outline",
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

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ColorStyle {
    pub variant: Option<Variant>,
}

impl ColorStyle {
    pub fn as_class(&self) -> &'static str {
        self.variant.as_ref().map(|v| v.as_class()).unwrap_or("")
    }
}
