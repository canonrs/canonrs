use leptos::prelude::*;

#[component]
pub fn CopyButton(
    #[prop(into)] text: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into)] id: String,
    #[prop(into, default = "Copy to clipboard".to_string())] aria_label: String,
) -> impl IntoView {
    view! {
        <button
            class=class
            id=id
            data-copy-button=""
            data-copy-text=text
            data-state="idle"
            aria-label=aria_label
        >
            <svg data-copy-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect width="14" height="14" x="8" y="8" rx="2" ry="2"/>
                <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>
            </svg>
            <span data-copy-text-label="">"Copy"</span>
            <svg data-copied-icon="" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20 6 9 17l-5-5"/>
            </svg>
            <span data-copied-text-label="" aria-live="polite">"Copied!"</span>
        </button>
    }
}
