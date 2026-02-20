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
}

#[component]
pub fn CalloutBlock(
    #[prop(default = CalloutType::Info)] variant: CalloutType,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] icon: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] container_class: String,
    children: Children,
) -> impl IntoView {
    let variant_str = variant.as_str();

    view! {
        <div
            class=container_class
            data-callout=""
            data-variant=variant_str
        >
            {icon.map(|i| view! {
                <div data-callout-icon="">{i()}</div>
            })}
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
