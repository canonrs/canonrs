use leptos::prelude::*;
use super::Checkbox;

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <Checkbox checked=false>
            "Accept terms and conditions"
        </Checkbox>
    }
}

#[component]
pub fn checked_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox checked=false>"Unchecked"</Checkbox>
            <Checkbox checked=true>"Checked"</Checkbox>
        </div>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox checked=false disabled=true>"Disabled unchecked"</Checkbox>
            <Checkbox checked=true disabled=true>"Disabled checked"</Checkbox>
        </div>
    }
}

#[component]
pub fn with_labels_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox name="newsletter".to_string() value="yes".to_string()>"Subscribe to newsletter"</Checkbox>
            <Checkbox name="notifications".to_string() value="yes".to_string()>"Enable notifications"</Checkbox>
            <Checkbox name="marketing".to_string() value="yes".to_string()>"Receive marketing emails"</Checkbox>
        </div>
    }
}
