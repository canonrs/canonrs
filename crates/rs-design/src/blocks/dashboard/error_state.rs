//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy dashboard components
//! @canon-owner: dashboard-team
//! @canon-target-date: 2025-03-15

//! ErrorState Block - Estado de erro para dashboard

use crate::tokens::animations::AnimationVariant;
use crate::tokens::semantic::SemanticColors;
use crate::ui::{Animate, Button, Card};
use leptos::prelude::*;

#[component]
pub fn ErrorState(
    #[prop(optional)] message: Option<String>,
    #[prop(optional)] on_retry: Option<Callback<()>>,
) -> impl IntoView {
    let message = message.unwrap_or_else(|| "Failed to load dashboard data".to_string());

    view! {
        <Animate variant=AnimationVariant::Shake>
            <Card class="p-12 text-center border-red-200 dark:border-red-900">
                <div class="flex flex-col items-center gap-4">
                    <div
                        class="w-16 h-16 rounded-full flex items-center justify-center"
                        style=format!("background-color: {}20", SemanticColors::ERROR)
                    >
                        <svg
                            class="w-8 h-8"
                            style=format!("color: {}", SemanticColors::ERROR)
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>

                    <div>
                        <h3 class="text-lg font-semibold mb-2">"Something went wrong"</h3>
                        <p
                            class="text-sm"
                            style=format!("color: {}", SemanticColors::ERROR)
                        >
                            {message}
                        </p>
                    </div>

                    {on_retry.map(|cb| view! {
                        <Button on:click=move |_| cb.run(())>
                            "Try Again"
                        </Button>
                    })}
                </div>
            </Card>
        </Animate>
    }
}
