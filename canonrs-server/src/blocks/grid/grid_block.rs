use leptos::prelude::*;

#[component]
pub fn Grid(
    #[prop(default = 3u8)] columns: u8,
    #[prop(default = String::new(), into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=class
            data-block="grid"
            data-block-version="1"
            data-columns=columns.to_string()
        >
            {children()}
        </div>
    }
}
