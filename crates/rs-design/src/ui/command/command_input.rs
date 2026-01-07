use leptos::prelude::*;

/// CommandInput - Search input for command palette
/// 
/// **Type:** Pure Component (Type 1)
/// **Tokens:** 100% Canonical
#[component]
pub fn CommandInput(
    /// Current search query
    #[prop(into)]
    value: Signal<String>,
    /// Callback when value changes
    #[prop(into)]
    on_input: Callback<String>,
    /// Placeholder text
    #[prop(optional)]
    placeholder: &'static str,
) -> impl IntoView {
    let placeholder = if placeholder.is_empty() {
        "Type a command..."
    } else {
        placeholder
    };
    
    view! {
        <input
            type="text"
            class="w-full px-4 py-3 bg-transparent border-b border-border focus:outline-none focus:border-primary text-lg"
            placeholder=placeholder
            prop:value=move || value.get()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                on_input.run(val);
            }
            autofocus
        />
    }
}
