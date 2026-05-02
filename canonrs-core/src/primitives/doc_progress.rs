//! @canon-level: strict
//! @canon-owner: primitives-team
//! DocProgress Primitive

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum DocProgressPosition {
    Top,
    #[default]
    Bottom,
}

impl DocProgressPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            DocProgressPosition::Top    => "top",
            DocProgressPosition::Bottom => "bottom",
        }
    }
}

#[component]
pub fn DocProgressPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(default = 0u8)] progress: u8,
) -> impl IntoView {
    let uid_dp_1 = crate::infra::uid::generate("dp");
    let progress_str = progress.to_string();
    view! {
        <div
            data-rs-doc-progress=""
            data-rs-uid=uid_dp_1
            data-rs-interaction="init"
            data-rs-progress=progress_str.clone()
            role="progressbar"
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=progress_str
            aria-label="Reading progress"
            class=class
        >
            <div data-rs-doc-progress-bar="" />
        </div>
    }
}

#[component]
pub fn DocProgressPortal(
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] scroll_target: String,
    #[prop(default = DocProgressPosition::Top)] position: DocProgressPosition,
) -> impl IntoView {
    let uid_dp_1 = crate::infra::uid::generate("dp");
    view! {
        <div
            data-rs-doc-progress-portal=""
            data-rs-uid=uid_dp_1
            data-rs-interaction="init"
            data-rs-scroll-target=scroll_target
            data-rs-position=position.as_str()
            aria-hidden="true"
            class=class
        >
            <div data-rs-doc-progress-bar="" />
        </div>
    }
}
