use leptos::prelude::*;

#[component]
pub fn InputOtpPrimitive(
    #[prop(default = String::new())] id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = 1)] maxlength: u32,
    #[prop(default = String::new())] pattern: String,
    #[prop(default = String::new())] inputmode: String,
    #[prop(default = String::new())] autocomplete: String,
) -> impl IntoView {
    view! {
        <input
            type="text"
            id={id}
            class={class}
            name={name}
            value={value}
            disabled={disabled}
            maxlength={maxlength}
            pattern={pattern}
            inputmode={inputmode}
            autocomplete={autocomplete}
            attr:data-input-otp=""
            attr:aria-disabled={if disabled { "true" } else { "false" }}
        />
    }
}
