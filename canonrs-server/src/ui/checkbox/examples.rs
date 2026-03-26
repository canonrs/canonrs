use leptos::prelude::*;
use super::Checkbox;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Checkbox checked=false>
            "Accept terms and conditions"
        </Checkbox>
    }
}

#[component]
pub fn CheckedExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox checked=false>"Unchecked"</Checkbox>
            <Checkbox checked=true>"Checked"</Checkbox>
        </div>
    }
}

#[component]
pub fn DisabledExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox checked=false disabled=true>"Disabled unchecked"</Checkbox>
            <Checkbox checked=true disabled=true>"Disabled checked"</Checkbox>
        </div>
    }
}

#[component]
pub fn WithLabelsExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Checkbox name="newsletter">"Subscribe to newsletter"</Checkbox>
            <Checkbox name="notifications">"Enable notifications"</Checkbox>
            <Checkbox name="marketing">"Receive marketing emails"</Checkbox>
        </div>
    }
}
