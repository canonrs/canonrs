use crate::blocks::LanguageToggle;
use leptos::prelude::*;

#[component]
pub fn LoginLayout(children: Children) -> impl IntoView {
    view! {
        <div class="relative w-full">
            <div class="absolute top-4 right-4">
                <LanguageToggle />
            </div>
            {children()}
        </div>
    }
}
