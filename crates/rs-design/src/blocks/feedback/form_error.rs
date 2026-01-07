//! FormError Block - Feedback de erro acessível

use crate::tokens::animations::AnimationVariant;
use crate::ui::Animate;
use leptos::prelude::*;

#[component]
pub fn FormError(
    #[prop(into)] message: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <Animate variant=AnimationVariant::Shake>
            <div
                role="alert"
                class=format!(
                    "text-sm text-destructive flex items-center gap-2 {}",
                    class
                )
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                >
                    <circle cx="12" cy="12" r="10"/>
                    <line x1="12" y1="8" x2="12" y2="12"/>
                    <line x1="12" y1="16" x2="12.01" y2="16"/>
                </svg>
                <span>{message}</span>
            </div>
        </Animate>
    }
}
