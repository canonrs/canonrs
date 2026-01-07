//! @canon-level: loose
//! @canon-exceptions: [#21]
//! @canon-justification: Hardcoded success color (legacy)
//! @canon-owner: feedback-team
//! @canon-target-date: 2025-03-01

//! FormSuccess Block - Feedback de sucesso

use crate::tokens::animations::AnimationVariant;
use crate::ui::Animate;
use leptos::prelude::*;

#[component]
pub fn FormSuccess(
    #[prop(into)] message: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <Animate variant=AnimationVariant::FadeIn>
            <div
                role="alert"
                class=format!(
                    "text-sm text-green-600 dark:text-green-400 flex items-center gap-2 {}",
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
                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
                    <polyline points="22 4 12 14.01 9 11.01"/>
                </svg>
                <span>{message}</span>
            </div>
        </Animate>
    }
}
