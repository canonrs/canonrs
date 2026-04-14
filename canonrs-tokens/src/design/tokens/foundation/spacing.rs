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

    // Component size scale — interactive element heights
    FamilyToken::new("size-input-sm",     "1.75rem"),   // 28px — small input/select
    FamilyToken::new("size-icon-sm",      "1rem"),      // 16px — small icon
    FamilyToken::new("size-icon-md",      "1.25rem"),   // 20px — medium icon
    FamilyToken::new("size-icon-lg",      "1.75rem"),   // 28px — large icon (state/empty)
    FamilyToken::new("size-nav-item",     "2rem"),      // 32px — nav/pagination item
    FamilyToken::new("size-cell",         "2rem"),      // 32px — calendar cell
    FamilyToken::new("size-color-picker", "16rem"),     // 256px — color picker panel
    FamilyToken::new("size-textarea-min", "6rem"),      // 96px — textarea min height

    // Button height scale — enterprise (fills gap in size scale)
    FamilyToken::new("size-button-xs",    "1.75rem"),   // 28px
    FamilyToken::new("size-button-sm",    "2rem"),      // 32px = size-md
    FamilyToken::new("size-button-md",    "2.25rem"),   // 36px — between size-md and size-lg
    FamilyToken::new("size-button-lg",    "2.75rem"),   // 44px — between size-lg and size-xl
    FamilyToken::new("size-button-xl",    "3.25rem"),   // 52px

    // Layout heights
    FamilyToken::new("size-header",       "3.5rem"),    // 56px = size-2xl
    FamilyToken::new("size-nav-sm",       "2.25rem"),   // 36px — calendar nav
];
