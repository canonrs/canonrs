//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy dashboard components
//! @canon-owner: dashboard-team
//! @canon-target-date: 2025-03-15

//! ActivityList Block - Lista de atividades recentes

use crate::tokens::animations::AnimationVariant;
use crate::ui::{Animate, Card};
use leptos::prelude::*;

#[component]
pub fn ActivityList(#[prop(optional)] loading: bool) -> impl IntoView {
    let activities = vec![
        ("New order #3245", "2 minutes ago"),
        ("User john@example.com signed up", "15 minutes ago"),
        ("Payment processed $450", "1 hour ago"),
        ("Report generated", "2 hours ago"),
    ];

    view! {
        <Animate variant=AnimationVariant::FadeUp>
            <Card>
                <div class="p-6">
                    <h3 class="text-lg font-semibold mb-4">"Recent Activity"</h3>

                    {if loading {
                        view! {
                            <div class="space-y-4">
                                {(0..4).map(|_| view! {
                                    <div class="animate-pulse flex gap-3">
                                        <div class="h-10 w-10 bg-gray-200 rounded-full"></div>
                                        <div class="flex-1">
                                            <div class="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
                                            <div class="h-3 bg-gray-200 rounded w-1/2"></div>
                                        </div>
                                    </div>
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-4">
                                {activities.into_iter().map(|(title, time)| view! {
                                    <div class="flex items-start gap-3 p-3 hover:bg-gray-50 dark:hover:bg-gray-800 rounded-lg transition-colors">
                                        <div class="h-10 w-10 rounded-full bg-primary-100 dark:bg-primary-900 flex items-center justify-center">
                                            <span class="text-primary-600 dark:text-primary-400">"•"</span>
                                        </div>
                                        <div class="flex-1">
                                            <p class="text-sm font-medium">{title}</p>
                                            <p class="text-xs text-gray-500">{time}</p>
                                        </div>
                                    </div>
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }}
                </div>
            </Card>
        </Animate>
    }
}
