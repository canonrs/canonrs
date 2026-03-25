//! @canon-level: strict
//! @canon-owner: primitives-team
//! Field Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn FieldPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <div data-rs-field="" class=class>{children()}</div> }
}

#[component]
pub fn FieldLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] html_for: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <label
            data-rs-field-label=""
            for={if html_for.is_empty() { None } else { Some(html_for) }}
            class=class
        >
            {children()}
        </label>
    }
}

#[component]
pub fn FieldDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <div data-rs-field-description="" class=class>{children()}</div> }
}

#[component]
pub fn FieldErrorPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-field-error="" role="alert" aria-live="polite" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn FieldGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <div data-rs-field-group="" class=class>{children()}</div> }
}

#[component]
pub fn FieldSetPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <fieldset data-rs-fieldset="" class=class>{children()}</fieldset> }
}

#[component]
pub fn FieldLegendPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <legend data-rs-field-legend="" class=class>{children()}</legend> }
}
