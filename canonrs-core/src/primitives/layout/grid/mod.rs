//! @canon-level: strict
//! @canon-owner: primitives-team
//! Grid Primitive - CSS Grid layout container

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum GridCols {
    One,
    Two,
    Three,
    #[default]
    Four,
    Six,
    Twelve,
    Auto,
}
impl GridCols {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::One    => "1",
            Self::Two    => "2",
            Self::Three  => "3",
            Self::Four   => "4",
            Self::Six    => "6",
            Self::Twelve => "12",
            Self::Auto   => "auto",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum GridGap {
    None,
    Xs,
    #[default]
    Sm,
    Md,
    Lg,
    Xl,
}
impl GridGap {
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
pub fn GridPrimitive(
    children: Children,
    #[prop(default = GridCols::Auto)] cols: GridCols,
    #[prop(default = GridGap::Sm)] gap: GridGap,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-grid=""
            data-rs-uid=crate::infra::uid::generate("gr")
            data-rs-cols=cols.as_str()
            data-rs-gap=gap.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

/// Construtor funcional — retorna AnyView sem passar pelo macro view!
pub fn grid_view(
    cols:     GridCols,
    gap:      GridGap,
    children: AnyView,
) -> AnyView {
    let uid = crate::infra::uid::generate("gr");
    view! {
        <div
            data-rs-grid=""
            data-rs-uid=uid
            data-rs-cols=cols.as_str()
            data-rs-gap=gap.as_str()
        >
            {children}
        </div>
    }.into_any()
}
