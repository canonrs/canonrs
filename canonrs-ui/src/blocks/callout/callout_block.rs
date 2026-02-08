//! # Callout Block
//! Enhanced alert with icons and variants

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CalloutType {
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl CalloutType {
    fn as_str(&self) -> &'static str {
        match self {
            CalloutType::Default => "default",
            CalloutType::Info => "info",
            CalloutType::Success => "success",
            CalloutType::Warning => "warning",
            CalloutType::Error => "error",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            CalloutType::Default => "📝",
            CalloutType::Info => "ℹ️",
            CalloutType::Success => "✅",
            CalloutType::Warning => "⚠️",
            CalloutType::Error => "🚫",
        }
    }
}

#[component]
pub fn CalloutBlock(
    #[prop(default = CalloutType::Info)] variant: CalloutType,
    #[prop(optional, into)] title: Option<String>,
    /// Container class for layout integration only (not for styling the block)
    #[prop(default = String::new(), into)] container_class: String,
    children: Children,
) -> impl IntoView {
    let icon = variant.icon();
    let variant_str = variant.as_str();

    view! {
        <div
            class=container_class
            data-callout=""
            data-variant=variant_str
        >
            <div data-callout-icon="">{icon}</div>
            <div>
                {title.map(|t| view! { 
                    <div data-callout-title="">{t}</div>
                })}
                <div data-callout-description="">
                    {children()}
                </div>
            </div>
        </div>
    }
}
