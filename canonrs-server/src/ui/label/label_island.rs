use leptos::prelude::*;
use super::label_ui::Label;

#[component]
pub fn LabelIsland(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] for_id: Option<String>,
    #[prop(optional)] _required: Option<bool>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <Label
            for_id=for_id.unwrap_or_default()
            class=class.unwrap_or_default()
        >
            {text.unwrap_or_default()}
        </Label>
    }
}
