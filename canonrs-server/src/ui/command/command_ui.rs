use leptos::prelude::*;

/// Command - UI Structure (SSR-safe)
/// No logic, just HTML structure
#[component]
pub fn Command(
    #[prop(into)] id: String,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] value: Option<Signal<String>>,
    children: Children,
) -> impl IntoView {
    let value_signal = value.unwrap_or_else(|| Signal::derive(|| String::new()));
    
    view! {
        <div data-command id=id>
            <input 
                data-command-input
                type="text"
                placeholder=placeholder.unwrap_or_else(|| "Search...".to_string())
                prop:value=move || value_signal.get()
            />
            <div data-command-list>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn CommandItem(
    #[prop(into)] text: String,
    #[prop(optional)] visible: Option<Signal<bool>>,
) -> impl IntoView {
    let is_visible = visible.unwrap_or_else(|| Signal::derive(|| true));
    
    view! {
        <div 
            data-command-item
            style:display=move || if is_visible.get() { "flex" } else { "none" }
        >
            {text}
        </div>
    }
}
