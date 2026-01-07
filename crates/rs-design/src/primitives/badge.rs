//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Internal primitive
//! @canon-owner: primitives-team
//! @canon-target-date: 2025-04-01

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Default,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BadgeSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn BadgePrimitive(
    #[prop(optional)] variant: Option<BadgeVariant>,
    #[prop(optional)] size: Option<BadgeSize>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(BadgeVariant::Default);
    let size = size.unwrap_or(BadgeSize::Md);

    view! {
        <span
            data-variant=match variant {
                BadgeVariant::Default => "default",
                BadgeVariant::Success => "success",
                BadgeVariant::Warning => "warning",
                BadgeVariant::Danger => "danger",
                BadgeVariant::Info => "info",
            }
            data-size=match size {
                BadgeSize::Sm => "sm",
                BadgeSize::Md => "md",
                BadgeSize::Lg => "lg",
            }
        >
            {children()}
        </span>
    }
}
