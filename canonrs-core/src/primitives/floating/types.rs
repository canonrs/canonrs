use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Placement {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

impl Placement {
    pub fn as_str(&self) -> &str {
        match self {
            Placement::Top => "top",
            Placement::TopStart => "top-start",
            Placement::TopEnd => "top-end",
            Placement::Bottom => "bottom",
            Placement::BottomStart => "bottom-start",
            Placement::BottomEnd => "bottom-end",
            Placement::Left => "left",
            Placement::LeftStart => "left-start",
            Placement::LeftEnd => "left-end",
            Placement::Right => "right",
            Placement::RightStart => "right-start",
            Placement::RightEnd => "right-end",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FloatingPosition {
    pub x: f64,
    pub y: f64,
    pub placement: Placement,
}

#[derive(Debug, Clone, Copy)]
pub struct FloatingConfig {
    pub placement: Placement,
    pub offset: f64,
    pub flip: bool,
}

impl Default for FloatingConfig {
    fn default() -> Self {
        Self {
            placement: Placement::Bottom,
            offset: 8.0,
            flip: true,
        }
    }
}
