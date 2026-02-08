use leptos::prelude::*;
use super::types::{Notification, NotificationPriority};
use crate::primitives::toast::ToastPrimitive;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
pub struct NotificationCenter {
    notifications: RwSignal<VecDeque<Notification>>,
    max_notifications: usize,
}

impl NotificationCenter {
    pub fn new(max_notifications: usize) -> Self {
        Self {
            notifications: RwSignal::new(VecDeque::new()),
            max_notifications,
        }
    }

    pub fn push(&self, notification: Notification) {
        let id = notification.id.clone();
        let duration = notification.duration;
        
        self.notifications.update(|queue| {
            // Remove oldest if at capacity
            if queue.len() >= self.max_notifications {
                queue.pop_front();
            }

            // Insert based on priority
            match notification.priority {
                NotificationPriority::Critical => queue.push_front(notification),
                NotificationPriority::High => {
                    let pos = queue.iter().position(|n| 
                        matches!(n.priority, NotificationPriority::Normal | NotificationPriority::Low)
                    ).unwrap_or(queue.len());
                    queue.insert(pos, notification);
                }
                _ => queue.push_back(notification),
            }
        });

        // Auto-dismiss if duration is set
        if let Some(dur) = duration {
            let notifications = self.notifications;
            set_timeout(
                move || {
                    notifications.update(|queue| {
                        queue.retain(|n| n.id != id);
                    });
                },
                dur,
            );
        }
    }

    pub fn dismiss(&self, id: &str) {
        self.notifications.update(|queue| {
            queue.retain(|n| n.id != id);
        });
    }

    pub fn clear(&self) {
        self.notifications.update(|queue| {
            queue.clear();
        });
    }

    pub fn notifications(&self) -> Signal<Vec<Notification>> {
        let notif = self.notifications;
        Signal::derive(move || {
            notif.get().iter().cloned().collect()
        })
    }
}

pub fn use_notification_center() -> NotificationCenter {
    use_context::<NotificationCenter>()
        .expect("NotificationCenter must be provided")
}

#[component]
pub fn NotificationCenterProvider(
    children: Children,
    #[prop(default = 5)] max_notifications: usize,
) -> impl IntoView {
    let center = NotificationCenter::new(max_notifications);
    provide_context(center);

    view! { {children()} }
}

#[component]
pub fn NotificationCenterView(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let center = use_notification_center();

    view! {
        <div
            class={format!("fixed top-4 right-4 z-50 flex flex-col gap-2 {}", class)}
            id={id}
        >
            {move || center.notifications().get().into_iter().map(|notification| {
                let type_class = match notification.notification_type {
                    super::types::NotificationType::Info => "bg-blue-500 text-white",
                    super::types::NotificationType::Success => "bg-green-500 text-white",
                    super::types::NotificationType::Warning => "bg-yellow-500 text-white",
                    super::types::NotificationType::Error => "bg-red-500 text-white",
                };

                view! {
                    <ToastPrimitive class={format!("min-w-80 p-4 rounded-lg shadow-lg {}", type_class)}>
                        <div class="flex items-start justify-between gap-3">
                            <div class="flex-1 space-y-1">
                                <h4 class="font-semibold">{notification.title}</h4>
                                <p class="text-sm opacity-90">{notification.message}</p>
                            </div>
                        </div>
                    </ToastPrimitive>
                }
            }).collect_view()}
        </div>
    }
}
