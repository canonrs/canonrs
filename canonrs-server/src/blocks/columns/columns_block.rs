use leptos::prelude::*;

#[component]
pub fn Columns(
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class data-block="columns" data-block-version="1">
            {children()}
        </div>
    }
}
