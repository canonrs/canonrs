//! Spacing contract — margin + padding via token scale

#[derive(Clone, Copy, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum SpaceScale {
    None,
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xxl,
}

impl SpaceScale {
    pub fn as_margin_class(&self) -> &'static str {
        match self {
            Self::None => "m-0",
            Self::Xs   => "m-xs",
            Self::Sm   => "m-sm",
            Self::Md   => "m-md",
            Self::Lg   => "m-lg",
            Self::Xl   => "m-xl",
            Self::Xxl  => "m-2xl",
        }
    }
    pub fn as_padding_class(&self) -> &'static str {
        match self {
            Self::None => "p-0",
            Self::Xs   => "p-xs",
            Self::Sm   => "p-sm",
            Self::Md   => "p-md",
            Self::Lg   => "p-lg",
            Self::Xl   => "p-xl",
            Self::Xxl  => "p-2xl",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "xs"   => Some(Self::Xs),
            "sm"   => Some(Self::Sm),
            "md"   => Some(Self::Md),
            "lg"   => Some(Self::Lg),
            "xl"   => Some(Self::Xl),
            "2xl"  => Some(Self::Xxl),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Spacing {
    pub margin:  Option<SpaceScale>,
    pub padding: Option<SpaceScale>,
}
