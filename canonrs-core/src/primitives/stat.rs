//! @canon-level: strict
//! @canon-owner: primitives-team
//! Stat Primitive - Number + label display


#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum StatSize {
    Sm,
    #[default]
    Md,
    Lg,
}
impl StatSize {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Sm => "sm", Self::Md => "md", Self::Lg => "lg" }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum StatTrend {
    #[default]
    Neutral,
    Increase,
    Decrease,
}
impl StatTrend {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Neutral => "neutral", Self::Increase => "increase", Self::Decrease => "decrease" }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum StatAlign {
    #[default]
    Start,
    Center,
    End,
}
impl StatAlign {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Start => "start", Self::Center => "center", Self::End => "end" }
    }
}

use leptos::prelude::*;

#[component]
pub fn StatPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_sta = crate::infra::uid::generate("sta");
    view! {
        <div
            data-rs-stat=""
            data-rs-uid=uid_sta
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn StatValuePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-stat-value="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn StatLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-stat-label="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn StatDeltaPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-stat-delta="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn StatIconPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-stat-icon="" aria-hidden="true" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn StatHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-stat-header="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn StatBodyPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-stat-body="" class=class>
            {children()}
        </div>
    }
}
