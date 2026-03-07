//! # ButtonGroup Block
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonGroupOrientation { Horizontal, Vertical }

impl ButtonGroupOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonGroupOrientation::Horizontal => "horizontal",
            ButtonGroupOrientation::Vertical => "vertical",
        }
    }
}

#[component]
pub fn ButtonGroupBlock(
    #[prop(default = ButtonGroupOrientation::Horizontal)] orientation: ButtonGroupOrientation,
    #[prop(default = false)] attached: bool,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!("canon-button-group canon-button-group--{} {} {}", orientation.as_str(), if attached { "canon-button-group--attached" } else { "" }, class)
            data-block="button-group"
            data-block-version="1"
            role="group"
        >
            <div data-block-region="buttons">
                {children()}
            </div>
        </div>
    }
}
