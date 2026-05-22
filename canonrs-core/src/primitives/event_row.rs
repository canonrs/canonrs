//! @canon-level: strict
//! @canon-owner: primitives-team
//! EventRow Primitive - substitui div.event-row + span

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum EventRowKind {
    #[default]
    Default,
    Info,
    Warn,
    Error,
    Success,
}
impl EventRowKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Info    => "info",
            Self::Warn    => "warn",
            Self::Error   => "error",
            Self::Success => "success",
        }
    }
}

#[component]
pub fn EventRowPrimitive(
    children: Children,
    #[prop(default = EventRowKind::Default)] kind: EventRowKind,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("evr");
    view! {
        <div
            data-rs-event-row=""
            data-rs-uid=uid
            data-rs-kind=kind.as_str()
            data-rs-layout="row"
            data-rs-slots="primary secondary meta"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn EventRowLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-event-row-label="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn EventRowTimePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-event-row-time="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn EventRowMessagePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-event-row-msg="" class=class>
            {children()}
        </span>
    }
}
