use leptos::prelude::*;

#[component]
pub fn FieldPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-rs-field="" class={class} id={if id.is_empty() { None } else { Some(id) }}>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn FieldLabelPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] html_for: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <label
            attr:data-rs-field-label=""
            for={if html_for.is_empty() { None } else { Some(html_for) }}
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
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
        <div data-rs-field-description="" class={class} id={if id.is_empty() { None } else { Some(id) }}>
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
            attr:data-rs-field-error=""
            role="alert"
            attr:aria-live="polite"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
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
        <div data-rs-field-group="" class={class} id={if id.is_empty() { None } else { Some(id) }}>
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
        <fieldset data-rs-fieldset="" class={class} id={if id.is_empty() { None } else { Some(id) }}>
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
        <legend data-rs-field-legend="" class={class} id={if id.is_empty() { None } else { Some(id) }}>
            {children.map(|c| c())}
        </legend>
    }
}
