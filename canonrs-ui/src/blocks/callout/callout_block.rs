//! # Callout Block
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum CalloutType { Default, Info, Success, Warning, Error }

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
    view! {
        <div
            class=container_class
            data-block="callout"
            data-block-version="1"
            data-variant=variant.as_str()
        >
            <div data-block-region="icon">
                {icon.map(|i| view! { <div data-callout-icon="">{i()}</div> })}
            </div>
            <div data-block-region="body">
                <div data-block-region="title">
                    {title.map(|t| view! { <div data-callout-title="">{t}</div> })}
                </div>
                <div data-block-region="description" class="canon-callout__description">
                    {children()}
                </div>
            </div>
        </div>
    }
}
