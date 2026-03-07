use leptos::prelude::*;
use super::InputOtp;

#[component]
pub fn basic_example() -> impl IntoView {
    view! {
        <InputOtp length=6 />
    }
}

#[component]
pub fn with_value_example() -> impl IntoView {
    view! {
        <InputOtp value="123456".to_string() length=6 />
    }
}

#[component]
pub fn custom_length_example() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div>
                <label>"4-digit OTP"</label>
                <InputOtp length=4 />
            </div>
            <div>
                <label>"8-digit OTP"</label>
                <InputOtp length=8 />
            </div>
        </div>
    }
}

#[component]
pub fn disabled_example() -> impl IntoView {
    view! {
        <InputOtp value="123456".to_string() length=6 disabled=true />
    }
}
