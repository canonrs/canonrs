use leptos::prelude::*;

#[component]
pub fn ShowcasePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-6">
            <div class="max-w-7xl mx-auto space-y-8">
                <header class="space-y-2">
                    <h1 class="text-4xl font-bold text-foreground">
                        "CanonRS Design System"
                    </h1>
                    <p class="text-lg text-muted-foreground">
                        "Modern, accessible, and composable UI components for Leptos"
                    </p>
                </header>

                <section class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div class="p-6 border rounded-lg">
                        <h2 class="text-xl font-semibold mb-2">"Components"</h2>
                        <p class="text-muted-foreground">
                            "Complete library of UI components"
                        </p>
                    </div>
                    <div class="p-6 border rounded-lg">
                        <h2 class="text-xl font-semibold mb-2">"Theme Engine"</h2>
                        <p class="text-muted-foreground">
                            "Customizable theming system"
                        </p>
                    </div>
                    <div class="p-6 border rounded-lg">
                        <h2 class="text-xl font-semibold mb-2">"Drag & Drop"</h2>
                        <p class="text-muted-foreground">
                            "Built-in drag and drop support"
                        </p>
                    </div>
                </section>
            </div>
        </div>
    }
}
