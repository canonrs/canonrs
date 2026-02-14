use leptos::prelude::*;
use crate::primitives::{
    CalloutPrimitive,
    CalloutIconPrimitive,
    CalloutTitlePrimitive,
    CalloutDescriptionPrimitive,
};

pub use crate::primitives::CalloutVariant;

#[component]
pub fn Callout(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = CalloutVariant::Default)] variant: CalloutVariant,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <CalloutPrimitive variant=variant class=class id=id>
            {children.map(|c| c())}
        </CalloutPrimitive>
    }
}

#[component]
pub fn CalloutIcon(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <CalloutIconPrimitive class=class id=id>
            {children.map(|c| c())}
        </CalloutIconPrimitive>
    }
}

#[component]
pub fn CalloutTitle(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <CalloutTitlePrimitive class=class id=id>
            {children.map(|c| c())}
        </CalloutTitlePrimitive>
    }
}

#[component]
pub fn CalloutDescription(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] id: String,
) -> impl IntoView {
    view! {
        <CalloutDescriptionPrimitive class=class id=id>
            {children.map(|c| c())}
        </CalloutDescriptionPrimitive>
    }
}
