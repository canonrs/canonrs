//! @canon-level: strict
//! @canon-owner: primitives-team
//! WorkerCard Primitive - substitui div.worker-card / div.noc-worker

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum WorkerCardVariant {
    #[default]
    Default,
    Noc,
    Compact,
}
impl WorkerCardVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Noc     => "noc",
            Self::Compact => "compact",
        }
    }
}

#[component]
pub fn WorkerCardPrimitive(
    children: Children,
    #[prop(default = WorkerCardVariant::Default)] variant: WorkerCardVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid = crate::infra::uid::generate("wkc");
    view! {
        <div
            data-rs-worker-card=""
            data-rs-uid=uid
            data-rs-variant=variant.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn WorkerCardHostPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-worker-card-host="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn WorkerCardIdPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-worker-card-id="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn WorkerCardStatsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-worker-card-stats="" class=class>
            {children()}
        </span>
    }
}
