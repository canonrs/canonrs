use leptos::prelude::*;

#[component]
pub fn ButtonGroupPrimitive(
    #[prop(optional)] id: Option<String>,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] attached: bool,
    #[prop(optional)] aria_label: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id={id}
            class={class}
            data-button-group=""
            attr:data-attached={if attached { Some("true") } else { None }}
            role="group"
            aria-label={aria_label}
        >
            {children.map(|c| c())}
        </div>
    }
}
