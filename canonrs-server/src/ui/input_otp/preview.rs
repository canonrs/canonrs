use leptos::prelude::*;
use super::input_otp_boundary::InputOtp;

#[component]
pub fn InputOtpShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InputOtp length=6 />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "OTP slots and active state managed automatically."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"4 digits"</span>
                <div data-rs-showcase-preview-row="">
                    <InputOtp length=4 />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Filled"</span>
                <div data-rs-showcase-preview-row="">
                    <InputOtp length=6 value="123456".to_string() />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Partial"</span>
                <div data-rs-showcase-preview-row="">
                    <InputOtp length=6 initial_value="123".to_string() />
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <InputOtp length=6 disabled=true />
                </div>
            </div>

        </div>
    }
}
