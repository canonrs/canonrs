// Foundation — Spacing + Size Scale
// Physical space and size values only.

use crate::design::tokens::FamilyToken;

pub const FOUNDATION_SPACING: &[FamilyToken] = &[
    // Size scale — used by all interactive components
    FamilyToken::new("size-2xs", "1.25rem"),
    FamilyToken::new("size-xs",  "1.5rem"),
    FamilyToken::new("size-sm",  "1.75rem"),
    FamilyToken::new("size-md",  "2rem"),
    FamilyToken::new("size-lg",  "2.5rem"),
    FamilyToken::new("size-xl",  "3rem"),
    FamilyToken::new("size-2xl", "3.5rem"),
    FamilyToken::new("size-3xl", "4rem"),

    // Space scale — layout rhythm
    FamilyToken::new("space-2xs", "0.125rem"),
    FamilyToken::new("space-xs",  "0.25rem"),
    FamilyToken::new("space-sm",  "0.5rem"),
    FamilyToken::new("space-md",  "0.75rem"),
    FamilyToken::new("space-lg",  "1rem"),
    FamilyToken::new("space-xl",  "1.5rem"),
    FamilyToken::new("space-2xl", "2rem"),
    FamilyToken::new("space-3xl", "3rem"),
    FamilyToken::new("space-4xl", "4rem"),
];
