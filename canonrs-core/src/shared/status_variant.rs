#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatusVariant {
    Online,
    Offline,
    Warning,
    Error,
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

impl Default for StatusVariant {
    fn default() -> Self {
        Self::Idle
    }
}
