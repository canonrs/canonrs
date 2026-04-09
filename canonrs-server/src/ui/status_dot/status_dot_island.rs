use leptos::prelude::*;

use canonrs_core::primitives::StatusDotVariant;

#[island]
pub fn StatusDotIsland(
    #[prop(optional)] variant: Option<StatusDotVariant>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class   = class.unwrap_or_default();
    let variant = variant.unwrap_or_default();
    let aria    = variant.aria_label();

    view! {
        <span data-rs-status-dot-wrapper="" style="display:inline-flex;align-items:center;gap:var(--space-xs);">
            <span
                data-rs-status-dot=""
                data-rs-component="StatusDot"
                data-rs-variant=variant.as_str()
                data-rs-state=variant.state()
                role="img"
                aria-label=aria
                class=class
            ></span>
            {label.map(|l| view! { <span data-rs-status-dot-label="" style="white-space:nowrap;">{l}</span> })}
        </span>
    }
}
