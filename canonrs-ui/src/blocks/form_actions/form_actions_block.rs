//! # FormActions Block
//! Action buttons container for forms

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum FormActionsAlignment {
    Left,
    Center,
    Right,
    Between,
}

impl FormActionsAlignment {
    fn as_str(&self) -> &'static str {
        match self {
            FormActionsAlignment::Left => "left",
            FormActionsAlignment::Center => "center",
            FormActionsAlignment::Right => "right",
            FormActionsAlignment::Between => "between",
        }
    }
}

#[component]
pub fn FormActionsBlock(
    #[prop(default = FormActionsAlignment::Right)] alignment: FormActionsAlignment,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div 
            class=format!("canon-form-actions canon-form-actions--{} {}", alignment.as_str(), class)
            attr:data-block="form-actions"
        >
            {children()}
        </div>
    }
}
