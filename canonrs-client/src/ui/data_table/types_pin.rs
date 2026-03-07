#[derive(Clone, Debug, PartialEq, Copy)]
pub enum PinPosition {
    Left,
    Right,
    None,
}

impl Default for PinPosition {
    fn default() -> Self {
        Self::None
    }
}
