//! Callout Island — Canon Rule passthrough
use leptos::prelude::*;
use canonrs_core::primitives::CalloutVariant;

#[component]
pub fn CalloutIsland(
    #[prop(optional, into)] title:       Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] icon:        Option<String>,
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(into, default = String::new())] class: String,
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
