#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::separator::SeparatorOrientation;
use canonrs_core::primitives::{
    FieldPrimitive, FieldSetPrimitive, FieldLabelPrimitive,
    FieldDescriptionPrimitive, FieldErrorPrimitive, FieldGroupPrimitive,
    FieldLegendPrimitive, SeparatorPrimitive,
    FieldErrorItemPrimitive, FieldSeparatorWrapperPrimitive, FieldSeparatorContentPrimitive,
    FieldContentPrimitive, FieldTitlePrimitive,
};
use super::variants::{FieldOrientation, FieldValidation};
use super::types::FieldLegendVariant;

#[component]
pub fn FieldSet(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <FieldSetPrimitive class=class>{children()}</FieldSetPrimitive> }
}
#[component]
pub fn FieldLegend(children: Children, #[prop(default = FieldLegendVariant::Legend)] variant: FieldLegendVariant, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <FieldLegendPrimitive attr:data-rs-variant=variant.as_str() class=class>{children()}</FieldLegendPrimitive> }
}
#[component]
pub fn FieldGroup(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <FieldGroupPrimitive class=class>{children()}</FieldGroupPrimitive> }
}
#[component]
pub fn Field(children: Children, #[prop(default = FieldOrientation::Vertical)] orientation: FieldOrientation, #[prop(default = FieldValidation::None)] _validation: FieldValidation, #[prop(default = false)] disabled: bool, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! {
        <FieldPrimitive attr:data-rs-orientation=orientation.as_str() attr:data-rs-validation=_validation.as_str() attr:data-rs-disabled=if disabled { "true" } else { "false" } class=class>
            {children()}
        </FieldPrimitive>
    }
}
#[component]
pub fn FieldContent(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <FieldContentPrimitive class=class>{children()}</FieldContentPrimitive> }
}
#[component]
pub fn FieldLabel(children: Children, #[prop(into, default = String::new())] html_for: String, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <FieldLabelPrimitive html_for=html_for class=class>{children()}</FieldLabelPrimitive> }
}
#[component]
pub fn FieldTitle(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <FieldTitlePrimitive class=class>{children()}</FieldTitlePrimitive> }
}
#[component]
pub fn FieldDescription(children: Children, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! { <FieldDescriptionPrimitive class=class>{children()}</FieldDescriptionPrimitive> }
}
#[component]
pub fn FieldError(#[prop(default = vec![])] errors: Vec<String>, #[prop(default = FieldValidation::None)] _validation: FieldValidation, #[prop(optional)] children: Option<Children>, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    if errors.is_empty() && children.is_none() { return view! {}.into_any(); }
    view! {
        <FieldErrorPrimitive class=class>
            {if let Some(child) = children { child().into_any() }
             else { view! { {errors.into_iter().map(|e| view! { <FieldErrorItemPrimitive>{e}</FieldErrorItemPrimitive> }).collect_view()} }.into_any() }}
        </FieldErrorPrimitive>
    }.into_any()
}
#[component]
pub fn FieldSeparator(#[prop(optional)] children: Option<Children>, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! {
        <FieldSeparatorWrapperPrimitive class=class>
            <SeparatorPrimitive orientation=SeparatorOrientation::Horizontal decorative=true class=String::new() />
            {children.map(|c| view! { <FieldSeparatorContentPrimitive>{c()}</FieldSeparatorContentPrimitive> })}
        </FieldSeparatorWrapperPrimitive>
    }
}
