use leptos::prelude::*;
use super::radio_group_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <RadioGroup>
            <RadioGroupItem value="option-1".to_string() checked=true>
                <RadioGroupIndicator />
                <span>"Option 1"</span>
            </RadioGroupItem>
            <RadioGroupItem value="option-2".to_string()>
                <RadioGroupIndicator />
                <span>"Option 2"</span>
            </RadioGroupItem>
            <RadioGroupItem value="option-3".to_string()>
                <RadioGroupIndicator />
                <span>"Option 3"</span>
            </RadioGroupItem>
        </RadioGroup>
    }
}
