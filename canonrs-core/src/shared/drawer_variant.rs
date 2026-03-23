#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Default)]
pub enum DrawerVariant {
    #[default]
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

