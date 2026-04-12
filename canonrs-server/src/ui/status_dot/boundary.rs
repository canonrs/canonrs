//! @canon-level: strict
//! StatusDot Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::status_dot_ui::StatusDot as StatusDotUi;
use canonrs_core::primitives::StatusDotVariant;

#[component]
pub fn StatusDot(
    #[prop(default = StatusDotVariant::Offline)] variant: StatusDotVariant,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-status-dot-wrapper="" style="display:inline-flex;align-items:center;gap:var(--space-xs);">
            <StatusDotUi variant=variant class=class>
                <span></span>
            </StatusDotUi>
            {label.map(|l| view! { <span data-rs-status-dot-label="">{l
};</span> })}
        </span>
    }
}
