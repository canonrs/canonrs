#![allow(unreachable_pub, dead_code)]
use leptos::prelude::*;
use canonrs_core::primitives::{
    FormErrorSummaryPrimitive
};

#[derive(Clone, Debug, PartialEq)]
pub struct FormError { pub field_label: String, pub message: String }

#[component]
pub fn FormErrorSummary(#[prop(default = vec![])] errors: Vec<FormError>, #[prop(into, default = "Please fix the following errors:".to_string())] title: String, #[prop(into, default = String::new())] class: String) -> impl IntoView {
    view! {
        <FormErrorSummaryPrimitive class=class>
            <>{title}</>
            <>
                {errors.iter().map(|e| view! {
                    <>
                        <>
                            {e.field_label.clone()}{": "}{e.message.clone()}
                        </>
                    </>
                }).collect_view()}
            </>
        </FormErrorSummaryPrimitive>
    }
}
