//! Callout Boundary — Canon Rule passthrough
use leptos::prelude::*;
use super::callout_ui::{Callout as CalloutUi, CalloutIcon, CalloutTitle, CalloutDescription};
pub use canonrs_core::primitives::CalloutVariant;

#[component]
pub fn Callout(
    #[prop(optional, into)] title:       Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] icon:        Option<String>,
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <CalloutUi variant=variant class=class>
            {icon.map(|i| view! { <CalloutIcon>{i}</CalloutIcon> })}
            {title.map(|t| view! { <CalloutTitle>{t}</CalloutTitle> })}
            {description.map(|d| view! { <CalloutDescription>{d}</CalloutDescription> })}
        </CalloutUi>
    }
}
