use leptos::prelude::*;
use leptos::ev::Event;

#[component]
pub fn ColorPicker(
    value: Signal<String>,
    on_change: impl Fn(String) + 'static,
    #[prop(optional)] name: Option<String>,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    let on_input = move |ev: Event| {
        let new_value = event_target_value(&ev);
        on_change(new_value);
    };
    
    view! {
        <div data-color-picker-wrapper="">
            <div 
                data-color-picker=""
                style=move || format!("background-color: {}", value.get())
            >
                <input
                    type="color"
                    value=move || value.get()
                    name=name.unwrap_or_default()
                    disabled=disabled
                    data-color-picker-input=""
                    id=id.unwrap_or_default()
                    on:input=on_input
                />
            </div>
        </div>
    }
}

#[component]
pub fn ColorPickerSwatch(
    color: String,
    #[prop(default = false)] selected: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-color-swatch=""
            data-selected=if selected { Some("") } else { None }
            style=format!("background-color: {}", color)
            class=class.unwrap_or_default()
            id=id.unwrap_or_default()
        />
    }
}
