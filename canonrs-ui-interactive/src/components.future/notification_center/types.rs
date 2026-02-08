use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub priority: NotificationPriority,
    pub duration: Option<Duration>,
    pub dismissible: bool,
}

impl Notification {
    pub fn new(id: impl Into<String>, title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            message: message.into(),
            notification_type: NotificationType::Info,
            priority: NotificationPriority::Normal,
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        }
    }

    pub fn notification_type(mut self, notification_type: NotificationType) -> Self {
        self.notification_type = notification_type;
        self
    }

    pub fn priority(mut self, priority: NotificationPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn duration(mut self, duration: Option<Duration>) -> Self {
        self.duration = duration;
        self
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }
}
