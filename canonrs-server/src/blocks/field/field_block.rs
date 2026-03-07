//! # Field Block
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum FieldValidation { None, Success, Warning, Error }

impl FieldValidation {
    fn as_str(&self) -> &'static str {
        match self {
            FieldValidation::None => "none",
            FieldValidation::Success => "success",
            FieldValidation::Warning => "warning",
            FieldValidation::Error => "error",
        }
    }
}

#[component]
pub fn FieldBlock(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(default = FieldValidation::None)] validation: FieldValidation,
    #[prop(default = false)] required: bool,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!("canon-field canon-field--{} {}", validation.as_str(), class)
            data-block="field"
            data-block-version="1"
        >
            <div data-block-region="label">
                {label.map(|l| view! {
                    <label class="canon-field__label">
                        {l}
                        {required.then(|| view! { <span class="canon-field__required">"*"</span> })}
                    </label>
                })}
            </div>
            <div data-block-region="input" class="canon-field__input">
                {children()}
            </div>
            <div data-block-region="description">
                {description.map(|d| view! { <div class="canon-field__description">{d}</div> })}
            </div>
            <div data-block-region="error">
                {error.map(|e| view! { <div class="canon-field__error">{e}</div> })}
            </div>
        </div>
    }
}
