//! @canon-level: strict
//! ColorPicker Primitive - native input[type=color]

use leptos::prelude::*;

#[component]
pub fn ColorPickerPrimitive(
    #[prop(into, default = "#000000".to_string())] value: String,
    #[prop(into, default = String::new())] name: String,
    #[prop(default = false)] disabled: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <input
            type="color"
            data-rs-color-picker=""
            value=value
            name=name
            disabled=disabled
            aria-label="Color picker"
            class=class
        />
    }
}

#[component]
pub fn ColorPickerTriggerPrimitive(
    children: ChildrenFn,
    #[prop(into, default = "#000000".to_string())] color: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let style = format!("background-color: {};", color);
    view! {
        <button
            type="button"
            data-rs-color-picker-trigger=""
            aria-label="Open color picker"
            aria-haspopup="dialog"
            class=class
        >
            <div data-rs-color-swatch="" style=style />
            {children()}
        </button>
    }
}

#[component]
pub fn ColorPickerSwatchPrimitive(
    #[prop(into, default = "#000000".to_string())] color: String,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let style = format!("background-color: {};", color);
    let aria = format!("Select color {}", color);
    view! {
        <button
            type="button"
            data-rs-color-swatch=""
            data-rs-selected={if selected { Some("true") } else { None }}
            aria-pressed={if selected { Some("true") } else { None }}
            style=style
            aria-label=aria
            class=class
        />
    }
}
