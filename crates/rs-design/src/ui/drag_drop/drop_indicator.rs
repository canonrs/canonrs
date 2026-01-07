use leptos::prelude::*;

/// DropIndicator - Visual line showing drop position
///
/// **Type:** Pure Component (Type 1)
#[component]
pub fn DropIndicator(
    /// Whether indicator is visible
    #[prop(into)]
    visible: Signal<bool>,
    
    /// Position: "top" or "bottom"
    #[prop(default = "top".to_string())]
    position: String,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                if visible.get() {
                    format!(
                        "absolute left-0 right-0 h-0.5 bg-primary pointer-events-none transition-opacity {}",
                        if position == "top" { "top-0" } else { "bottom-0" }
                    )
                } else {
                    "hidden".to_string()
                }
            }
        />
    }
}
