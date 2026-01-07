use leptos::prelude::*;
use crate::primitives::checkbox::CheckboxPrimitive;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] indeterminate: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(default = Callback::new(|_| {}))] on_checked_change: Callback<bool>,
    #[prop(optional)] class: String,
) -> impl IntoView {
    view! {
        <CheckboxPrimitive
            checked=checked
            indeterminate=indeterminate
            disabled=disabled
            on_checked_change=on_checked_change
            class=format!(
                "grid place-content-center h-4 w-4 shrink-0 \
                rounded-[var(--radius-sm)] \
                border border-primary \
                shadow-[var(--shadow-sm)] \
                transition-colors duration-[var(--motion-duration-fast)] \
                focus-visible:outline-none \
                focus-visible:ring-1 \
                focus-visible:ring-[hsl(var(--state-focus-ring))] \
                disabled:cursor-not-allowed \
                disabled:opacity-[var(--state-disabled-opacity)] \
                data-[state=checked]:bg-primary \
                data-[state=checked]:text-primary-foreground \
                data-[state=indeterminate]:bg-primary \
                data-[state=indeterminate]:text-primary-foreground \
                {}",
                class
            )
        >
            <svg
                class=move || format!(
                    "h-4 w-4 transition-opacity {}",
                    if indeterminate.get() || checked.get() { "opacity-100" } else { "opacity-0" }
                )
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="3"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                {move || {
                    if indeterminate.get() {
                        view! { <line x1="5" y1="12" x2="19" y2="12" /> }.into_any()
                    } else {
                        view! { <polyline points="20 6 9 17 4 12" /> }.into_any()
                    }
                }}
            </svg>
        </CheckboxPrimitive>
    }
}
