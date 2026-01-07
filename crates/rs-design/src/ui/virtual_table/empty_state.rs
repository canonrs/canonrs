use leptos::prelude::*;

#[component]
pub fn EmptyState(
    #[prop(into, optional)] message: Option<String>,
) -> impl IntoView {
    let msg = message.unwrap_or_else(|| "No data available".to_string());
    
    view! {
        <div class="flex items-center justify-center h-full py-12">
            <div class="text-center">
                <svg 
                    class="mx-auto h-12 w-12 text-fg-muted"
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24"
                >
                    <path 
                        stroke-linecap="round" 
                        stroke-linejoin="round" 
                        stroke-width="2" 
                        d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"
                    />
                </svg>
                <h3 class="mt-2 text-sm font-medium text-fg-default">
                    {msg}
                </h3>
            </div>
        </div>
    }
}
