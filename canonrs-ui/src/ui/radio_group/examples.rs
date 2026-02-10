use leptos::prelude::*;
use super::{RadioGroup, RadioGroupItem, RadioGroupIndicator};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <RadioGroup id="radio-basic">
            <RadioGroupItem id="radio-1" name="radio-basic" value="option-1".to_string() checked=true>
                "Option 1"
            </RadioGroupItem>
            <RadioGroupItem id="radio-2" name="radio-basic" value="option-2".to_string()>
                "Option 2"
            </RadioGroupItem>
            <RadioGroupItem id="radio-3" name="radio-basic" value="option-3".to_string()>
                "Option 3"
            </RadioGroupItem>
        </RadioGroup>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <RadioGroup id="radio-disabled">
            <RadioGroupItem id="radio-d1" name="radio-disabled" value="option-1".to_string() checked=true>
                "Enabled"
            </RadioGroupItem>
            <RadioGroupItem id="radio-d2" name="radio-disabled" value="option-2".to_string() disabled=true>
                "Disabled"
            </RadioGroupItem>
        </RadioGroup>
    }
}

#[component]
pub fn with_labels_example() -> impl IntoView {
    view! {
        <RadioGroup id="radio-labels">
            <RadioGroupItem id="radio-sm" name="size" value="small".to_string()>
                "Small"
            </RadioGroupItem>
            <RadioGroupItem id="radio-md" name="size" value="medium".to_string() checked=true>
                "Medium"
            </RadioGroupItem>
            <RadioGroupItem id="radio-lg" name="size" value="large".to_string()>
                "Large"
            </RadioGroupItem>
        </RadioGroup>
    }
}
