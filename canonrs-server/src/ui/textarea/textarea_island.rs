use leptos::prelude::*;
use super::textarea_ui::Textarea;
use canonrs_core::meta::DisabledState;

#[island]
pub fn TextareaIsland(
    #[prop(optional, into)] value: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] readonly: Option<bool>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional, into)] aria_labelledby: Option<String>,
    #[prop(optional, into)] aria_describedby: Option<String>,
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let disabled_state = if disabled.unwrap_or(false) {
        DisabledState::Disabled
    } else {
        DisabledState::Enabled
    };

    view! {
        <Textarea
            value=value.unwrap_or_default()
            placeholder=placeholder.unwrap_or_default()
            name=name.unwrap_or_default()
            disabled=disabled_state
            readonly=readonly.unwrap_or(false)
            required=required.unwrap_or(false)
            aria_labelledby=aria_labelledby.unwrap_or_default()
            aria_describedby=aria_describedby.unwrap_or_default()
            rows=rows.unwrap_or(3)
            class=class.unwrap_or_default()
        />
    }
}
