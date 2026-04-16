//! @canon-level: strict
//! @canon-owner: primitives-team
//! TableOfContents Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TocMode {
    #[default]
    Simple,
    Expand,
    Nested,
}

impl TocMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Simple => "simple",
            Self::Expand => "expand",
            Self::Nested => "nested",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TocItemState {
    #[default]
    Idle,
    Active,
    Ancestor,
}

impl TocItemState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle     => "idle",
            Self::Active   => "active",
            Self::Ancestor => "ancestor",
        }
    }
}

#[component]
pub fn TocPrimitive(
    children: Children,
    #[prop(default = TocMode::Simple)] mode: TocMode,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-toc=""
            data-rs-uid=crate::infra::uid::generate("toc")
            data-rs-interaction="init"
            data-rs-mode=mode.as_str()
            aria-label="Table of contents"
            class=class
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn TocTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-toc-title="" class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn TocListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ul data-rs-toc-list="" class=class>
            {children()}
        </ul>
    }
}

#[component]
pub fn TocSubtreePrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ul
            data-rs-toc-subtree=""
            data-rs-state=state.as_str()
            class=class
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn TocItemPrimitive(
    children: Children,
    #[prop(into)] data_level: String,
    #[prop(into, default = String::new())] data_target: String,
    #[prop(default = TocItemState::Idle)] state: TocItemState,
    #[prop(default = false)] is_child: bool,
    #[prop(default = false)] has_children: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <li
            data-rs-toc-item=""
            data-rs-level=data_level
            data-rs-target=data_target
            data-rs-state=state.as_str()
            data-rs-child=if is_child { "true" } else { "false" }
            data-rs-has-children=if has_children { "true" } else { "false" }
            class=class
        >
            {children()}
        </li>
    }
}

#[component]
pub fn TocLinkPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a data-rs-toc-link="" href=href class=class>
            {children()}
        </a>
    }
}

#[component]
pub fn TocExpandButtonPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let open = state == VisibilityState::Open;
    view! {
        <button
            type="button"
            data-rs-toc-expand-btn=""
            data-rs-state=state.as_str()
            aria-expanded=if open { "true" } else { "false" }
            class=class
        >
            {children()}
        </button>
    }
}
