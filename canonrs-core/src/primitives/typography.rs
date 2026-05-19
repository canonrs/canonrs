//! @canon-level: strict
//! @canon-owner: primitives-team
//! Typography Primitives — governed text semantics
//! Text, Heading, Caption, Metric, Code (inline), Strong, Muted

use leptos::prelude::*;

// ── Variants ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TextSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}
impl TextSize {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Xs => "xs", Self::Sm => "sm", Self::Md => "md",
                     Self::Lg => "lg", Self::Xl => "xl" }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TextWeight {
    #[default]
    Normal,
    Medium,
    Bold,
}
impl TextWeight {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Normal => "normal", Self::Medium => "medium", Self::Bold => "bold" }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TextVariant {
    #[default]
    Default,
    Muted,
    Primary,
    Success,
    Warning,
    Destructive,
}
impl TextVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default     => "default",
            Self::Muted       => "muted",
            Self::Primary     => "primary",
            Self::Success     => "success",
            Self::Warning     => "warning",
            Self::Destructive => "destructive",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum HeadingLevel {
    #[default]
    H2,
    H1,
    H3,
    H4,
    H5,
    H6,
}
impl HeadingLevel {
    pub fn as_str(&self) -> &'static str {
        match self { Self::H1 => "1", Self::H2 => "2", Self::H3 => "3",
                     Self::H4 => "4", Self::H5 => "5", Self::H6 => "6" }
    }
}

// ── Primitives ────────────────────────────────────────────────────────────────

/// TextPrimitive — governed inline text
#[component]
pub fn TextPrimitive(
    children: Children,
    #[prop(default = TextSize::Md)]    size:    TextSize,
    #[prop(default = TextWeight::Normal)] weight: TextWeight,
    #[prop(default = TextVariant::Default)] variant: TextVariant,
    #[prop(default = false)] mono: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-text=""
            data-rs-size=size.as_str()
            data-rs-weight=weight.as_str()
            data-rs-variant=variant.as_str()
            data-rs-mono=if mono { Some("") } else { None }
            class=class
        >
            {children()}
        </span>
    }
}

/// HeadingPrimitive — governed heading h1-h6
#[component]
pub fn HeadingPrimitive(
    children: Children,
    #[prop(default = HeadingLevel::H2)] level: HeadingLevel,
    #[prop(default = TextSize::Lg)]     size:  TextSize,
    #[prop(default = TextWeight::Bold)] weight: TextWeight,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let l = level.as_str();
    view! {
        <span
            data-rs-heading=""
            data-rs-level=l
            data-rs-size=size.as_str()
            data-rs-weight=weight.as_str()
            role="heading"
            attr:aria-level=l
            class=class
        >
            {children()}
        </span>
    }
}

/// CaptionPrimitive — small auxiliary text
#[component]
pub fn CaptionPrimitive(
    children: Children,
    #[prop(default = TextVariant::Muted)] variant: TextVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-caption=""
            data-rs-variant=variant.as_str()
            class=class
        >
            {children()}
        </span>
    }
}

/// MetricPrimitive — numeric value display
#[component]
pub fn MetricPrimitive(
    children: Children,
    #[prop(default = TextSize::Xl)]     size:    TextSize,
    #[prop(default = TextWeight::Bold)] weight:  TextWeight,
    #[prop(default = TextVariant::Default)] variant: TextVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-metric=""
            data-rs-size=size.as_str()
            data-rs-weight=weight.as_str()
            data-rs-variant=variant.as_str()
            class=class
        >
            {children()}
        </span>
    }
}

/// CodeInlinePrimitive — inline code
#[component]
pub fn CodeInlinePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span
            data-rs-code-inline=""
            class=class
        >
            {children()}
        </span>
    }
}
