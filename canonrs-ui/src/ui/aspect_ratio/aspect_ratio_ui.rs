use leptos::prelude::*;
use crate::primitives::AspectRatio as AspectRatioPrimitive;

#[component]
pub fn AspectRatio(
    children: Children,
    #[prop(default = 16.0)] _width: f32,
    #[prop(default = 9.0)] _height: f32,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <AspectRatioPrimitive 
            class={class.unwrap_or_default()} 
            id={id.unwrap_or_default()}
        >
            {children()}
        </AspectRatioPrimitive>
    }
}
