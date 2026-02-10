use leptos::prelude::*;
use crate::primitives::{
    FieldPrimitive, FieldSetPrimitive, FieldLabelPrimitive,
    FieldDescriptionPrimitive, FieldErrorPrimitive, FieldGroupPrimitive,
    FieldLegendPrimitive, SeparatorPrimitive,
};
use super::variants::{FieldOrientation, FieldValidation};
use super::types::FieldLegendVariant;

#[component]
pub fn FieldSet(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <FieldSetPrimitive class={class} id={id.unwrap_or_default()}>
            {children()}
        </FieldSetPrimitive>
    }
}

#[component]
pub fn FieldLegend(
    children: Children,
    #[prop(default = FieldLegendVariant::Legend)] variant: FieldLegendVariant,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <FieldLegendPrimitive
            attr:data-variant={variant.as_str()}
            class={class}
            id={id.unwrap_or_default()}
        >
            {children()}
        </FieldLegendPrimitive>
    }
}

#[component]
pub fn FieldGroup(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <FieldGroupPrimitive class={class} id={id.unwrap_or_default()}>
            {children()}
        </FieldGroupPrimitive>
    }
}

#[component]
pub fn Field(
    children: Children,
    #[prop(default = FieldOrientation::Vertical)] orientation: FieldOrientation,
    #[prop(default = FieldValidation::None)] validation: FieldValidation,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <FieldPrimitive
            attr:data-orientation={orientation.as_str()}
            attr:data-validation={validation.as_str()}
            attr:data-disabled={if disabled { "true" } else { "false" }}
            class={class}
            id={id.unwrap_or_default()}
        >
            {children()}
        </FieldPrimitive>
    }
}

#[component]
pub fn FieldContent(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    let id_str = id.unwrap_or_default();
    view! {
        <div
            attr:data-field-content=""
            class={class}
            id={if id_str.is_empty() { None } else { Some(id_str) }}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldLabel(
    children: Children,
    #[prop(into, optional)] html_for: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <FieldLabelPrimitive
            html_for={html_for.unwrap_or_default()}
            class={class}
            id={id.unwrap_or_default()}
        >
            {children()}
        </FieldLabelPrimitive>
    }
}

#[component]
pub fn FieldTitle(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    let id_str = id.unwrap_or_default();
    view! {
        <div
            attr:data-field-title=""
            class={class}
            id={if id_str.is_empty() { None } else { Some(id_str) }}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldDescription(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <FieldDescriptionPrimitive class={class} id={id.unwrap_or_default()}>
            {children()}
        </FieldDescriptionPrimitive>
    }
}

#[component]
pub fn FieldError(
    #[prop(default = vec![])] errors: Vec<String>,
    #[prop(default = FieldValidation::None)] validation: FieldValidation,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    if errors.is_empty() && children.is_none() {
        return view! {}.into_any();
    }
    view! {
        <FieldErrorPrimitive
            attr:data-validation={validation.as_str()}
            class={class}
            id={id.unwrap_or_default()}
        >
            {if let Some(child) = children {
                view! { {child()} }.into_any()
            } else if errors.len() == 1 {
                view! { <span>{errors[0].clone()}</span> }.into_any()
            } else {
                view! {
                    <ul data-field-error-list="">
                        {errors.into_iter().map(|e| view! { <li>{e}</li> }).collect_view()}
                    </ul>
                }.into_any()
            }}
        </FieldErrorPrimitive>
    }.into_any()
}

#[component]
pub fn FieldSeparator(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    let id_str = id.unwrap_or_default();
    view! {
        <div
            attr:data-field-separator-wrapper=""
            class={class}
            id={if id_str.is_empty() { None } else { Some(id_str) }}
        >
            <SeparatorPrimitive
                orientation={"horizontal".to_string()}
                decorative={true}
                class={String::new()}
                id={String::new()}
            />
            {children.map(|content| view! {
                <span attr:data-field-separator-content="">{content()}</span>
            })}
        </div>
    }
}
