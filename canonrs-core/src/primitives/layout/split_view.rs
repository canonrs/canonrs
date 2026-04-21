//! @canon-level: strict
//! @canon-owner: primitives-team
//! SplitView layout ratio enum

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SplitRatio {
    #[default]
    Equal,
    FormFocused,
    ContextFocused,
}
impl SplitRatio {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Equal         => "50-50",
            Self::FormFocused   => "40-60",
            Self::ContextFocused => "60-40",
        }
    }
}
