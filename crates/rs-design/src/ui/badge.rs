use crate::primitives::{
    BadgePrimitive, BadgeSize as PrimitiveBadgeSize, BadgeVariant as PrimitiveBadgeVariant,
};
use crate::tokens::{SEMANTIC, SPACING};
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Badge(
    #[prop(default = PrimitiveBadgeVariant::Default)] variant: PrimitiveBadgeVariant,
    #[prop(default = PrimitiveBadgeSize::Md)] size: PrimitiveBadgeSize,
    #[prop(default = String::new(), into)] _class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <BadgePrimitive variant=variant size=size>
            {children()}
        </BadgePrimitive>
    }
}
