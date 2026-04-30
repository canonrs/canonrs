//! Textarea Island — Canon Rule #340
//! Passthrough only. Zero logic, zero transformation.

use leptos::prelude::*;
use super::textarea_ui::Textarea as TextareaUi;
use canonrs_core::meta::DisabledState;

#[component]
pub fn Textarea(
    #[prop(into, default = String::new())] value:            String,
    #[prop(into, default = String::new())] placeholder:      String,
    #[prop(into, default = String::new())] name:             String,
    #[prop(default = DisabledState::Enabled)] disabled:      DisabledState,
    #[prop(default = false)] readonly:                       bool,
    #[prop(default = false)] required:                       bool,
    #[prop(into, default = String::new())] aria_labelledby:  String,
    #[prop(into, default = String::new())] aria_describedby: String,
    #[prop(default = 3u32)] rows:                            u32,
    #[prop(into, default = String::new())] class:            String,
) -> impl IntoView {
    view! {
        <TextareaUi
            value=value
            placeholder=placeholder
            name=name
            disabled=disabled
            readonly=readonly
            required=required
            aria_labelledby=aria_labelledby
            aria_describedby=aria_describedby
            rows=rows
            class=class
        />
    }
}
