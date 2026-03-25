//! TableOfContents Primitives - Semantic structural wrappers
//! Pure SSR components, zero state, zero effects

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::state_engine::visibility_attrs;

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

#[component]
pub fn TocPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(default = TocMode::Simple)] mode: TocMode,
) -> impl IntoView {
    view! {
        <nav
            data-rs-toc=""
            data-rs-component="TableOfContents"
            data-rs-behavior="navigation"
            data-rs-mode=mode.as_str()
            id={if id.is_empty() { None } else { Some(id) }}
            class={class}
            aria-label="Table of contents"
        >
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn TocTitlePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <p data-rs-toc-title="" class={class}>
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn TocListPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ul data-rs-toc-list="" class={class}>
            {children.map(|c| c())}
        </ul>
    }
}

#[component]
pub fn TocSubtreePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <ul
            data-rs-toc-subtree=""
            data-rs-state=s.data_rs_state
            class={class}
        >
            {children.map(|c| c())}
        </ul>
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
pub fn TocItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into)] data_level: String,
    #[prop(into)] data_target: String,
    #[prop(default = TocItemState::Idle)] state: TocItemState,
    #[prop(default = false)] is_child: bool,
    #[prop(default = false)] has_children: bool,
) -> impl IntoView {
    view! {
        <li
            data-rs-toc-item=""
            data-rs-level=data_level
            data-rs-target=data_target
            data-rs-state=state.as_str()
            data-rs-child={if is_child { Some("true") } else { None }}
            data-rs-has-children={if has_children { Some("true") } else { None }}
            class={class}
        >
            {children.map(|c| c())}
        </li>
    }
}

#[component]
pub fn TocLinkPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <a data-rs-toc-link="" href={href} class={class}>
            {children.map(|c| c())}
        </a>
    }
}

#[component]
pub fn TocExpandButtonPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <button
            type="button"
            data-rs-toc-expand-btn=""
            data-rs-state=s.data_rs_state
            aria-expanded=s.aria_expanded
            class={class}
        >
            {children.map(|c| c())}
        </button>
    }
}
