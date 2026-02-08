use leptos::prelude::*;
use super::Switch;

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <Switch checked=false />
    }
}

#[component]
pub fn checked_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <Switch checked=false />
                <span>"Off"</span>
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <Switch checked=true />
                <span>"On"</span>
            </div>
        </div>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <Switch checked=false disabled=true />
                <span>"Disabled off"</span>
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <Switch checked=true disabled=true />
                <span>"Disabled on"</span>
            </div>
        </div>
    }
}

#[component]
pub fn with_labels_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <Switch name="airplane-mode".to_string() value="on".to_string() />
                <label>"Airplane Mode"</label>
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <Switch name="bluetooth".to_string() value="on".to_string() checked=true />
                <label>"Bluetooth"</label>
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <Switch name="wifi".to_string() value="on".to_string() checked=true />
                <label>"Wi-Fi"</label>
            </div>
        </div>
    }
}
