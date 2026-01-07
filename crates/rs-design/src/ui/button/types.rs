/// Button HTML type attribute
#[derive(Clone, Copy, Default)]
pub enum ButtonType {
    Button,
    #[default]
    Submit,
    Reset,
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}
