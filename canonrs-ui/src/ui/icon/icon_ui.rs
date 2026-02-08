use leptos::prelude::*;

#[component]
pub fn Icon(
    children: Children,
    #[prop(default = "md")] size: &'static str,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <span
            attr:data-icon-wrapper=""
            attr:data-size={size}
            class={class.unwrap_or_default()}
            id={id.unwrap_or_default()}
        >
            {children()}
        </span>
    }
}
