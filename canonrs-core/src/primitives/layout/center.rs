//! @canon-level: strict
//! @canon-owner: primitives-team
//! Center Primitive - Centers content

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum CenterMode {
    #[default]
    Horizontal,
    Vertical,
    Both,
}
impl CenterMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical   => "vertical",
            Self::Both       => "both",
        }
    }
}

#[component]
pub fn CenterPrimitive(
    children: Children,
    #[prop(default = CenterMode::Both)] mode: CenterMode,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-center=""
            data-rs-uid=crate::infra::uid::generate("cn")
            data-rs-mode=mode.as_str()
            class=class
        >
            {children()}
        </div>
    }
}
