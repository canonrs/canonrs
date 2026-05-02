//! @canon-level: strict
//! @canon-owner: primitives-team
//! CopyButton Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn CopyButtonPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] target: Option<String>,
    #[prop(default = 2000)] reset_delay: u32,
    #[prop(into, default = "Copy to clipboard".to_string())] aria_label: String,
) -> impl IntoView {
    let uid_cpb = crate::infra::uid::generate("cpb");
    view! {
        <button
            class=class
            id=id
            data-rs-copy-button=""
            data-rs-uid=uid_cpb
            data-rs-interaction="content"
            data-rs-copy-text=text
            data-rs-copy-target=target
            data-rs-reset-delay=reset_delay.to_string()
            data-rs-state="idle"
            aria-label=aria_label
        >
            <span data-rs-copy-content="">
                <svg data-rs-copy-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
                    <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
                </svg>
                <span data-rs-copy-label="">"Copy"</span>
            </span>

            <span data-rs-copied-content="">
                <svg data-rs-copied-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M20 6 9 17l-5-5"/>
                </svg>
                <span data-rs-copied-label="" role="status" aria-live="polite" aria-atomic="true">"Copied!"</span>
            </span>

            <span data-rs-error-content="">
                <svg data-rs-error-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"/>
                    <path d="m15 9-6 6M9 9l6 6"/>
                </svg>
                <span data-rs-error-label="" role="status" aria-live="assertive" aria-atomic="true">"Failed"</span>
            </span>
        </button>
    }
}
