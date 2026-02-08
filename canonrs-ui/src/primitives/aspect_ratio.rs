use leptos::prelude::*;

#[component]
pub fn AspectRatio(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::from("16/9"))] ratio: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div attr:data-aspect-ratio="" style=format!("aspect-ratio: {}", ratio) class={class} id={id}>
            {children.map(|c| c())}
        </div>
    }
}
