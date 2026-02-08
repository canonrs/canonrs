use leptos::callback::Callback;
use leptos::prelude::*;
use leptos::ev;
use canonrs_ui::ui::input::{Input, InputVariant, InputSize};

#[component]
pub fn InputInteractive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = InputVariant::Default)] variant: InputVariant,
    #[prop(default = InputSize::Md)] size: InputSize,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] aria_label: String,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_blur: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <Input
            class=class
            id=id
            input_type=input_type
            name=name
            value=value
            disabled=disabled
            variant=variant
            size=size
            placeholder=placeholder
            aria_label=aria_label
            on:input=move |ev: ev::Event| {
                if let Some(ref h) = on_input {
                    h.run(event_target_value(&ev));
                }
            }
            on:change=move |ev: ev::Event| {
                if let Some(ref h) = on_change {
                    h.run(event_target_value(&ev));
                }
            }
            on:blur=move |_ev: ev::FocusEvent| {
                if let Some(ref h) = on_blur {
                    h.run(());
                }
            }
        />
    }
}
