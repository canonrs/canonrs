use leptos::prelude::*;
use super::{ColorPicker, ColorPickerSwatch};

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <ColorPicker value="#3b82f6".to_string() />
    }
}

#[component]
pub fn with_swatches_example() -> impl IntoView {
    let colors = vec!["#ef4444", "#f59e0b", "#10b981", "#3b82f6", "#8b5cf6"];

    view! {
        <div style="display: flex; gap: 0.5rem;">
            {colors.into_iter().map(|color| {
                view! {
                    <ColorPickerSwatch color=color.to_string() />
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <ColorPicker value="#6b7280".to_string() disabled=true />
    }
}
