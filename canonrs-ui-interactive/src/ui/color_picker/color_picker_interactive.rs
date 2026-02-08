use leptos::callback::Callback;
use leptos::prelude::*;
use canonrs_ui::ui::color_picker::ColorPickerSwatch;

#[component]
pub fn ColorPickerSwatchInteractive(
    color: String,
    #[prop(default = false)] selected: bool,
    #[prop(optional)] on_select: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let color_clone = color.clone();

    view! {
        <ColorPickerSwatch
            color=color
            selected=selected
            class=class.unwrap_or_default()
            id=id.unwrap_or_default()
            on:click=move |_| {
                if let Some(ref handler) = on_select {
                    handler.run(color_clone.clone());
                }
            }
        />
    }
}
