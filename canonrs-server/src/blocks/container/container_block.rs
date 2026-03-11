use leptos::prelude::*;

#[component]
pub fn Container(
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class data-block="container" data-block-version="1">
            {children()}
        </div>
    }
}
