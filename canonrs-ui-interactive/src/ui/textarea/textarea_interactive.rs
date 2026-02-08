use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::ui::textarea::Textarea;

#[component]
pub fn TextareaInteractive(
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional)] labelled_by: Option<String>,
    #[prop(optional)] described_by: Option<String>,
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <Textarea
            value=value
            placeholder=placeholder
            name=name
            disabled=disabled
            readonly=readonly
            required=required
            labelled_by=labelled_by.unwrap_or_default()
            described_by=described_by.unwrap_or_default()
            rows=rows.unwrap_or(3u32)
            class=class
            id=id
            on:input=move |ev: ev::Event| {
                if let Some(ref cb) = on_input {
                    cb.run(event_target_value(&ev));
                }
            }
        />
    }
}
