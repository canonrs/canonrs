//! # ButtonGroup Block
//! Group of related buttons

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonGroupOrientation {
    Horizontal,
    Vertical,
}

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
            class=format!(
                "canon-button-group canon-button-group--{} {} {}",
                orientation.as_str(),
                if attached { "canon-button-group--attached" } else { "" },
                class
            )
            attr:data-block="button-group"
            role="group"
        >
            {children()}
        </div>
    }
}
