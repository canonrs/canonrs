use leptos::prelude::*;

#[component]
pub fn IconMoreHorizontal() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            class="h-4 w-4"
            fill="currentColor"
            stroke="none"
        >
            <circle cx="4" cy="12" r="2.5" />
            <circle cx="12" cy="12" r="2.5" />
            <circle cx="20" cy="12" r="2.5" />
        </svg>
    }
}
