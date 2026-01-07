use leptos::prelude::*;
use leptos::ev::Event;
use super::types::{MaskType, MaskedInputVariant};
use super::variants::*;

fn apply_mask(value: &str, mask_type: &MaskType) -> String {
    let pattern = mask_type.pattern();
    let digits: String = value.chars().filter(|c| c.is_numeric()).collect();
    
    let mut result = String::new();
    let mut digit_index = 0;
    
    for ch in pattern.chars() {
        if ch == '9' {
            if let Some(digit) = digits.chars().nth(digit_index) {
                result.push(digit);
                digit_index += 1;
            } else {
                break;
            }
        } else {
            if digit_index < digits.len() {
                result.push(ch);
            }
        }
    }
    
    result
}

#[component]
pub fn MaskedInput(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] mask_type: MaskType,
    #[prop(into, optional)] variant: MaskedInputVariant,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] disabled: bool,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let mask_type_clone = mask_type.clone();
    let max_len = mask_type.max_length();
    
    let handle_input = move |ev: Event| {
        let raw_value = event_target_value(&ev);
        let masked_value = apply_mask(&raw_value, &mask_type_clone);
        on_change.run(masked_value);
    };

    let base_classes = "
        h-[2.5rem]
        px-[0.5rem]
        text-[1rem]
        font-[400]
        rounded-[0.375rem]
        border-[1px]
        bg-surface
        text-fg-default
        placeholder:text-fg-muted
        transition-colors
        duration-[300ms]
        ease-[cubic-bezier(0.4,0.0,0.2,1)]
    ";

    let field_classes = format!(
        "{} {} {}",
        base_classes,
        variant.border_color(),
        variant.focus_ring()
    );

    let disabled_classes = if disabled {
        "opacity-[0.5] cursor-not-allowed"
    } else {
        "hover:opacity-[0.8]"
    };

    view! {
        <input
            type="text"
            class=format!("{} {} {}", field_classes, disabled_classes, class)
            value=move || value.get()
            on:input=handle_input
            placeholder=placeholder.unwrap_or_default()
            disabled=disabled
            maxlength=max_len
        />
    }
}
