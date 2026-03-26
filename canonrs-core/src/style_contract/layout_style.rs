//! Layout style contract — align + width

#[derive(Clone, Copy, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
    Between,
    Stretch,
}

impl Align {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Start   => "items-start",
            Self::Center  => "items-center",
            Self::End     => "items-end",
            Self::Between => "justify-between",
            Self::Stretch => "items-stretch",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "start"   => Some(Self::Start),
            "center"  => Some(Self::Center),
            "end"     => Some(Self::End),
            "between" => Some(Self::Between),
            "stretch" => Some(Self::Stretch),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum Width {
    #[default]
    Auto,
    Full,
    Container,
    Narrow,
    Wide,
}

impl Width {
    pub fn as_class(&self) -> &'static str {
        match self {
            Self::Auto      => "w-auto",
            Self::Full      => "w-full",
            Self::Container => "w-container",
            Self::Narrow    => "w-narrow",
            Self::Wide      => "w-wide",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "auto"      => Some(Self::Auto),
            "full"      => Some(Self::Full),
            "container" => Some(Self::Container),
            "narrow"    => Some(Self::Narrow),
            "wide"      => Some(Self::Wide),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LayoutStyle {
    pub align: Option<Align>,
    pub width: Option<Width>,
}
