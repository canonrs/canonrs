//! @canon-level: strict
//! @canon-owner: primitives-team
//! Field Primitive - HTML puro para forms

use leptos::prelude::*;

#[component]
pub fn FieldPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-field="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn FieldLabelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] html_for: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <label
            attr:data-field-label=""
            for={html_for.unwrap_or_default()}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </label>
    }
}

#[component]
pub fn FieldDescriptionPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-field-description="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn FieldErrorPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-field-error=""
            role="alert"
            attr:aria-live="polite"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn FieldGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-field-group="" class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn FieldSetPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <fieldset data-fieldset="" class={class} id={id}>
            {children.map(|c| c())}
        </fieldset>
    }
}

#[component]
pub fn FieldLegendPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <legend data-field-legend="" class={class} id={id}>
            {children.map(|c| c())}
        </legend>
    }
}
