//! @canon-level: strict
//! Alert Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::alert_ui::{
    Alert as AlertUi,
    AlertTitle,
    AlertDescription,
    AlertCloseButton
};
pub use canonrs_core::primitives::AlertVariant;

#[component]
pub fn Alert(
    #[prop(into, optional)] title: Option<String>,
    #[prop(into, optional)] description: Option<String>,
    #[prop(default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(default = false)] dismissible: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <AlertUi variant=variant class=class>
            {title.map(|t| view! { <AlertTitle>{t}</AlertTitle> })}
            {description.map(|d| view! { <AlertDescription>{d}</AlertDescription> })}
            {dismissible.then(|| view! { <AlertCloseButton>"×"</AlertCloseButton> })}
        </AlertUi>
    }
}
