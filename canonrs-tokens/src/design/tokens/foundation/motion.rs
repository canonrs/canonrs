// Foundation — Motion
// Duration and easing scale.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_MOTION: &[FamilyToken] = &[
    FamilyToken::new("motion-duration-instant",   "0ms"),
    FamilyToken::new("motion-duration-fast",      "120ms"),
    FamilyToken::new("motion-duration-normal",    "200ms"),
    FamilyToken::new("motion-duration-slow",      "300ms"),
    FamilyToken::new("motion-duration-deliberate","500ms"),
    FamilyToken::new("motion-ease-standard",      "cubic-bezier(0.2, 0, 0, 1)"),
    FamilyToken::new("motion-ease-in",            "cubic-bezier(0.4, 0, 1, 1)"),
    FamilyToken::new("motion-ease-out",           "cubic-bezier(0, 0, 0.2, 1)"),
    FamilyToken::new("motion-ease-linear",        "linear"),
    FamilyToken::new("motion-reduced-duration",   "0ms"),
];
