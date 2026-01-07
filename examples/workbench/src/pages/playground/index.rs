use leptos::prelude::*;

#[component]
pub fn PlaygroundPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background p-6">
            <div class="max-w-7xl mx-auto">
                <h1 class="text-3xl font-bold mb-6">"Interactive Playground"</h1>
                <p class="text-muted-foreground">
                    "Experiment with components and workflows"
                </p>
            </div>
        </div>
    }
}
