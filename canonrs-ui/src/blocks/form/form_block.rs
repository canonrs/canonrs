//! # Form Block

use leptos::prelude::*;
use leptos::ev;

#[derive(Clone, Copy, PartialEq)]
pub enum FormLayout { Vertical, Horizontal, Inline }

impl FormLayout {
    fn as_str(&self) -> &'static str {
        match self {
            FormLayout::Vertical => "vertical",
            FormLayout::Horizontal => "horizontal",
            FormLayout::Inline => "inline",
        }
    }
}

#[component]
pub fn FormBlock(
    #[prop(default = FormLayout::Vertical)] layout: FormLayout,
    #[prop(optional)] on_submit: Option<Callback<ev::SubmitEvent>>,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(cb) = on_submit { cb.run(ev); }
    };

    view! {
        <form
            class=format!("canon-form canon-form--{} {}", layout.as_str(), class)
            data-block="form"
            data-block-version="1"
            on:submit=handle_submit
        >
            <div data-block-region="fields">{children()}</div>
            <div data-block-region="actions"></div>
        </form>
    }
}
