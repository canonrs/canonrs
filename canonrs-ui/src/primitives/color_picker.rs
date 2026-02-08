//! @canon-level: strict
//! @canon-owner: primitives-team
//! ColorPicker Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn ColorPickerPrimitive(
    #[prop(default = "#000000".to_string())] value: String,
    #[prop(default = String::new())] name: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <input
            type="color"
            attr:data-color-picker=""
            value={value}
            name={name}
            disabled={disabled}
            attr:aria-label="Color picker"
            class={class}
            id={id}
        />
    }
}

#[component]
pub fn ColorPickerTriggerPrimitive(
    children: ChildrenFn,
    #[prop(default = "#000000".to_string())] color: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-color-picker-trigger=""
            type="button"
            attr:aria-label="Open color picker"
            class={class}
            id={id}
        >
            <div
                attr:data-color-swatch=""
                style={format!("background-color: {};", color)}
            />
            {children()}
        </button>
    }
}

#[component]
pub fn ColorPickerSwatchPrimitive(
    #[prop(default = "#000000".to_string())] color: String,
    #[prop(default = false)] selected: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-color-swatch=""
            attr:data-selected={selected}
            type="button"
            style={format!("background-color: {};", color)}
            attr:aria-label={format!("Select color {}", color)}
            class={class}
            id={id}
        />
    }
}
