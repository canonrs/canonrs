use leptos::prelude::*;

#[component]
pub fn AuthLayout(
    children: Children,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!(
            "flex min-h-screen w-full items-center justify-center p-6 md:p-10 {}",
            class
        )>
            <div class="w-full max-w-sm">
                {children()}
            </div>
        </div>
    }
}
