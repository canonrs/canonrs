#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Default)]
pub enum StatusVariant {
    Online,
    Offline,
    Warning,
    Error,
    #[default]
    Idle,
}

impl StatusVariant {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Idle => "idle",
        }
    }
}

