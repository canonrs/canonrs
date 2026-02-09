use leptos::prelude::*;
use super::Checkbox;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Checkbox id="checkbox-basic".to_string() checked=false>
            "Accept terms and conditions"
        </Checkbox>
    }
}

#[component]
pub fn checked_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox id="checkbox-unchecked".to_string() checked=false>"Unchecked"</Checkbox>
            <Checkbox id="checkbox-checked".to_string() checked=true>"Checked"</Checkbox>
        </div>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox id="checkbox-disabled-1".to_string() checked=false disabled=true>"Disabled unchecked"</Checkbox>
            <Checkbox id="checkbox-disabled-2".to_string() checked=true disabled=true>"Disabled checked"</Checkbox>
        </div>
    }
}

#[component]
pub fn with_labels_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox id="checkbox-newsletter".to_string() name="newsletter".to_string() value="yes".to_string()>"Subscribe to newsletter"</Checkbox>
            <Checkbox id="checkbox-notifications".to_string() name="notifications".to_string() value="yes".to_string()>"Enable notifications"</Checkbox>
            <Checkbox id="checkbox-marketing".to_string() name="marketing".to_string() value="yes".to_string()>"Receive marketing emails"</Checkbox>
        </div>
    }
}
