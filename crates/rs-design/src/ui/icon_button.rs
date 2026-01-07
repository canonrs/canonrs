use leptos::prelude::*;

#[component]
pub fn IconButton(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class=format!(
                "inline-flex items-center justify-center h-8 w-8 rounded-md transition-colors hover:bg-muted {}",
                class
            )
        >
            {children()}
        </button>
    }
}
