use crate::primitives::{AvatarFallbackPrimitive, AvatarImagePrimitive, AvatarPrimitive};
use crate::tokens::SEMANTIC;
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Avatar(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        "relative flex size-8 shrink-0 overflow-hidden rounded-full",
        class
    );

    view! {
        <AvatarPrimitive>
            <span class=merged_class>
                {children()}
            </span>
        </AvatarPrimitive>
    }
}

#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(default = "Avatar".to_string(), into)] alt: String,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!("aspect-square size-full object-cover", class);

    view! {


            <img src=src alt=alt class=merged_class />

    }
}

#[component]
pub fn AvatarFallback(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        format!(
            "bg-[{}] flex size-full items-center justify-center rounded-full text-sm font-medium",
            SEMANTIC.muted
        ),
        class
    );

    view! {
        <AvatarFallbackPrimitive>
            <span class=merged_class>
                {children()}
            </span>
        </AvatarFallbackPrimitive>
    }
}
