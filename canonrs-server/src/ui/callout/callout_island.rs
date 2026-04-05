use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum CalloutIslandVariant {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Destructive,
}

impl CalloutIslandVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Info        => "info",
            Self::Success     => "success",
            Self::Warning     => "warning",
            Self::Destructive => "destructive",
        }
    }
}

#[island]
pub fn CalloutIsland(
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional)] variant: Option<CalloutIslandVariant>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class   = class.unwrap_or_default();
    let variant = variant.unwrap_or_default();

    view! {
        <div
            data-rs-callout=""
            data-rs-component="Callout"
            data-rs-variant=variant.as_str()
            role="note"
            class=class
        >
            {icon.map(|i| view! { <span data-rs-callout-icon="">{i}</span> })}
            <div data-rs-callout-body="">
                {title.map(|t| view! { <p data-rs-callout-title="">{t}</p> })}
                {description.map(|d| view! { <p data-rs-callout-description="">{d}</p> })}
            </div>
        </div>
    }
}
