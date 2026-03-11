use leptos::prelude::*;

#[component]
pub fn Stack(
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class data-block="stack" data-block-version="1">
            {children()}
        </div>
    }
}
