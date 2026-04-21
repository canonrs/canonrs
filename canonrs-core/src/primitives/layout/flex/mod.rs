//! @canon-level: strict
//! @canon-owner: primitives-team
//! Flex Primitive - Free-form flex layout

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FlexJustify {
    #[default]
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}
impl FlexJustify {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start   => "start",
            Self::End     => "end",
            Self::Center  => "center",
            Self::Between => "between",
            Self::Around  => "around",
            Self::Evenly  => "evenly",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FlexAlign {
    #[default]
    Stretch,
    Start,
    Center,
    End,
    Baseline,
}
impl FlexAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stretch  => "stretch",
            Self::Start    => "start",
            Self::Center   => "center",
            Self::End      => "end",
            Self::Baseline => "baseline",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}
impl FlexDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Row           => "row",
            Self::Column        => "column",
            Self::RowReverse    => "row-reverse",
            Self::ColumnReverse => "column-reverse",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum FlexGap {
    None,
    Xs,
    #[default]
    Sm,
    Md,
    Lg,
    Xl,
}
impl FlexGap {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Xs   => "xs",
            Self::Sm   => "sm",
            Self::Md   => "md",
            Self::Lg   => "lg",
            Self::Xl   => "xl",
        }
    }
}

#[component]
pub fn FlexPrimitive(
    children: Children,
    #[prop(default = FlexDirection::Row)] direction: FlexDirection,
    #[prop(default = FlexJustify::Start)] justify: FlexJustify,
    #[prop(default = FlexAlign::Stretch)] align: FlexAlign,
    #[prop(default = FlexGap::Sm)] gap: FlexGap,
    #[prop(default = false)] wrap: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-flex=""
            data-rs-uid=crate::infra::uid::generate("fx")
            data-rs-direction=direction.as_str()
            data-rs-justify=justify.as_str()
            data-rs-align=align.as_str()
            data-rs-gap=gap.as_str()
            data-rs-wrap=wrap.to_string()
            class=class
        >
            {children()}
        </div>
    }
}
