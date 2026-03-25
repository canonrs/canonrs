use leptos::prelude::*;
use super::{RadioGroup, RadioGroupItem};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <RadioGroup>
            <RadioGroupItem name="radio-basic" value="option-1" checked=true>
                "Option 1"
            </RadioGroupItem>
            <RadioGroupItem name="radio-basic" value="option-2">
                "Option 2"
            </RadioGroupItem>
            <RadioGroupItem name="radio-basic" value="option-3">
                "Option 3"
            </RadioGroupItem>
        </RadioGroup>
    }
}

#[component]
pub fn DisabledExample() -> impl IntoView {
    view! {
        <RadioGroup>
            <RadioGroupItem name="radio-disabled" value="option-1" checked=true>
                "Enabled"
            </RadioGroupItem>
            <RadioGroupItem name="radio-disabled" value="option-2" disabled=true>
                "Disabled"
            </RadioGroupItem>
        </RadioGroup>
    }
}
