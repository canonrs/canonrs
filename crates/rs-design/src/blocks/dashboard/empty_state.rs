//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy dashboard components
//! @canon-owner: dashboard-team
//! @canon-target-date: 2025-03-15

//! EmptyState Block - Estado vazio para dashboard

use crate::tokens::animations::AnimationVariant;
use crate::ui::{Animate, Button, Card};
use leptos::prelude::*;

#[component]
pub fn EmptyState(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] action_label: Option<String>,
    #[prop(optional)] on_action: Option<Callback<()>>,
) -> impl IntoView {
    let title = title.unwrap_or_else(|| "No data available".to_string());
    let description =
        description.unwrap_or_else(|| "Get started by adding your first item".to_string());

    view! {
        <Animate variant=AnimationVariant::FadeIn>
            <Card class="p-12 text-center">
                <div class="flex flex-col items-center gap-4">
                    <div class="w-16 h-16 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center">
                        <svg class="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" />
                        </svg>
                    </div>

                    <div>
                        <h3 class="text-lg font-semibold mb-2">{title}</h3>
                        <p class="text-sm text-gray-600 dark:text-gray-400">{description}</p>
                    </div>

                    {action_label.map(|label| view! {
                        <Button on:click=move |_| {
                            if let Some(cb) = &on_action {
                                cb.run(());
                            }
                        }>
                            {label}
                        </Button>
                    })}
                </div>
            </Card>
        </Animate>
    }
}
