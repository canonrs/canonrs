//! @canon-id: status-dot
//! @canon-label: Status Dot
//! @canon-family: data_display
//! @canon-category: Display
//! @canon-intent: Indicate user presence or availability
//! @canon-description: Status indicator dot
//! @canon-composable: false
//! @canon-capabilities:
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: status-dot, status, indicator, online, offline, active

use leptos::prelude::*;
use canonrs_core::primitives::StatusDotPrimitive;
pub use canonrs_core::primitives::StatusDotVariant;

#[component]
pub fn StatusDot(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = StatusDotVariant::Offline)] variant: StatusDotVariant,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <StatusDotPrimitive variant=variant class={class.unwrap_or_default()}>
            {children.map(|c| c())}
        </StatusDotPrimitive>
    }
}

#[component]
pub fn StatusDotPreview() -> impl IntoView {
    view! {
        <div style="display:flex;align-items:center;gap:0.75rem;">
            <StatusDot variant=StatusDotVariant::Online />
            <StatusDot variant=StatusDotVariant::Offline />
            <StatusDot variant=StatusDotVariant::Away />
            <StatusDot variant=StatusDotVariant::Busy />
            <StatusDot variant=StatusDotVariant::DoNotDisturb />
        </div>
    }
}
