use leptos::prelude::*;

#[component]
pub fn Textarea(
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] placeholder: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] readonly: bool,
    #[prop(default = false)] required: bool,
    #[prop(optional, into)] labelled_by: Option<String>,
    #[prop(optional, into)] described_by: Option<String>,
    #[prop(optional, into)] rows: Option<u32>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <textarea
            attr:data-textarea=""
            prop:value={value}
            placeholder={placeholder}
            name={name}
            attr:aria-disabled={if disabled { "true" } else { "false" }}
            attr:data-disabled={if disabled { "true" } else { "" }}
            attr:aria-readonly={if readonly { "true" } else { "false" }}
            attr:data-readonly={if readonly { "true" } else { "" }}
            attr:aria-required={if required { "true" } else { "false" }}
            attr:data-required={if required { "true" } else { "" }}
            attr:aria-labelledby={labelled_by}
            attr:aria-describedby={described_by}
            rows={rows}
            class=class
            id=id
        />
    }
}
