//! @canon-level: strict
//! @canon-owner: primitives-team
//! Stack Primitive - Flex layout container (vertical or horizontal)

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum StackDirection {
    #[default]
    Vertical,
    Horizontal,
}
impl StackDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical   => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum StackAlign {
    #[default]
    Stretch,
    Start,
    Center,
    End,
}
impl StackAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stretch => "stretch",
            Self::Start   => "start",
            Self::Center  => "center",
            Self::End     => "end",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum StackGap {
    None,
    Xs,
    #[default]
    Sm,
    Md,
    Lg,
    Xl,
}
impl StackGap {
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
pub fn StackPrimitive(
    children: Children,
    #[prop(default = StackDirection::Vertical)] direction: StackDirection,
    #[prop(default = StackAlign::Stretch)] align: StackAlign,
    #[prop(default = StackGap::Sm)] gap: StackGap,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-stack=""
            data-rs-uid=crate::infra::uid::generate("sk")
            data-rs-direction=direction.as_str()
            data-rs-align=align.as_str()
            data-rs-gap=gap.as_str()
            class=class
        >
            {children()}
        </div>
    }
}
