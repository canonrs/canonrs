#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerVariant {
    Persistent,
    Modal,
}

impl DrawerVariant {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Persistent => "persistent",
            Self::Modal => "modal",
        }
    }
}

impl Default for DrawerVariant {
    fn default() -> Self {
        Self::Persistent
    }
}
