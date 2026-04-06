//! @canon-level: strict
//! Input Island — wrapper SSR puro
//! Input é nativo — sem estado semântico, sem DOM mutation

use leptos::prelude::*;
use super::input_ui::Input;

#[island]
pub fn InputIsland(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] input_type: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <Input
            class=class.unwrap_or_default()
            input_type=input_type.unwrap_or_else(|| "text".to_string())
            name=name.unwrap_or_default()
            value=value.unwrap_or_default()
            placeholder=placeholder.unwrap_or_default()
            aria_label=aria_label.unwrap_or_default()
        />
    }
}
