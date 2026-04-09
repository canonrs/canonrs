//! ButtonGroup Island — Canon Rule passthrough
use leptos::prelude::*;
use canonrs_core::ToggleState;
use super::button_group_ui::ButtonGroup;

#[component]
pub fn ButtonGroupIsland(
    children: Children,
    #[prop(default = ToggleState::Off)] attached: ToggleState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <ButtonGroup class=class attached=attached aria_label=aria_label.unwrap_or_default()>
            {children()}
        </ButtonGroup>
    }
}
