use leptos::prelude::*;
use super::{RadioGroup, RadioGroupItem};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <RadioGroup id="radio-basic">
            <RadioGroupItem id="radio-1" name="radio-basic" value="option-1" checked=true>
                "Option 1"
            </RadioGroupItem>
            <RadioGroupItem id="radio-2" name="radio-basic" value="option-2">
                "Option 2"
            </RadioGroupItem>
            <RadioGroupItem id="radio-3" name="radio-basic" value="option-3">
                "Option 3"
            </RadioGroupItem>
        </RadioGroup>
    }
}

#[component]
pub fn DisabledExample() -> impl IntoView {
    view! {
        <RadioGroup id="radio-disabled">
            <RadioGroupItem id="radio-d1" name="radio-disabled" value="option-1" checked=true>
                "Enabled"
            </RadioGroupItem>
            <RadioGroupItem id="radio-d2" name="radio-disabled" value="option-2" disabled=true>
                "Disabled"
            </RadioGroupItem>
        </RadioGroup>
    }
}
