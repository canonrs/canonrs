//! Typography contract — size + weight + align

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TextSize {
    Xs, Sm, #[default] Md, Lg, Xl, Xxl, Display,
}

impl TextSize {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Xs      => "text-xs",
            Self::Sm      => "text-sm",
            Self::Md      => "text-md",
            Self::Lg      => "text-lg",
            Self::Xl      => "text-xl",
            Self::Xxl     => "text-2xl",
            Self::Display => "text-display",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "xs" => Some(Self::Xs), "sm" => Some(Self::Sm),
            "md" => Some(Self::Md), "lg" => Some(Self::Lg),
            "xl" => Some(Self::Xl), "2xl" => Some(Self::Xxl),
            "display" => Some(Self::Display), _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TextWeight {
    #[default] Normal, Medium, Semibold, Bold,
}

impl TextWeight {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Normal   => "font-normal",
            Self::Medium   => "font-medium",
            Self::Semibold => "font-semibold",
            Self::Bold     => "font-bold",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "normal" => Some(Self::Normal), "medium" => Some(Self::Medium),
            "semibold" => Some(Self::Semibold), "bold" => Some(Self::Bold),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TextAlign {
    #[default] Left, Center, Right, Justify,
}

impl TextAlign {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Left    => "text-left",
            Self::Center  => "text-center",
            Self::Right   => "text-right",
            Self::Justify => "text-justify",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "left" => Some(Self::Left), "center" => Some(Self::Center),
            "right" => Some(Self::Right), "justify" => Some(Self::Justify),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Typography {
    pub size:   Option<TextSize>,
    pub weight: Option<TextWeight>,
    pub align:  Option<TextAlign>,
}
