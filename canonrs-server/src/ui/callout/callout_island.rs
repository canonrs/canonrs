//! Callout Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum CalloutIslandVariant {
    #[default] Default,
    Info, Success, Warning, Destructive,
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

#[component]
pub fn CalloutIsland(
    #[prop(optional, into)] title:                          Option<String>,
    #[prop(optional, into)] description:                    Option<String>,
    #[prop(optional, into)] icon:                           Option<String>,
    #[prop(default = CalloutIslandVariant::Default)] variant: CalloutIslandVariant,
    #[prop(into, default = String::new())] class:           String,
) -> impl IntoView {
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
