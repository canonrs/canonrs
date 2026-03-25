//! @canon-level: strict
//! @canon-owner: primitives-team
//! Separator Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical   => "vertical",
        }
    }
}

#[component]
pub fn SeparatorPrimitive(
    #[prop(default = SeparatorOrientation::Horizontal)] orientation: SeparatorOrientation,
    #[prop(default = true)] decorative: bool,
    #[prop(default = String::new())] aria_label: String,
    #[prop(default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let role = if decorative { "presentation" } else { "separator" };
    let aria_orientation = if !decorative { Some(orientation.as_str()) } else { None };
    let aria_label_val = if !decorative && !aria_label.is_empty() { Some(aria_label) } else { None };

    view! {
        <div
            data-rs-separator=""
            data-orientation=orientation.as_str()
            role=role
            aria-orientation=aria_orientation
            aria-label=aria_label_val
            class=class
            id=id
        />
    }
}
