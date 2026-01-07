use leptos::prelude::*;

#[component]
pub fn ThemePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-6">
            <div class="max-w-7xl mx-auto">
                <h1 class="text-3xl font-bold mb-6">"Theme Customization"</h1>
                <p class="text-muted-foreground">
                    "Customize colors, density, and more"
                </p>
            </div>
        </div>
    }
}
