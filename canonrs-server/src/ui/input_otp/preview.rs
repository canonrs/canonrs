use leptos::prelude::*;
use super::input_otp_boundary::InputOtp;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn InputOtpShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <InputOtp length=6 />
            <p data-rs-showcase-preview-anchor()>
                "OTP slots and active state managed automatically."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"4 digits"</span>
                <InputOtp length=4 />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Filled"</span>
                <InputOtp length=6 value="123456".to_string() />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Partial"</span>
                <InputOtp length=6 initial_value="123".to_string() />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <InputOtp length=6 disabled=true />
            </Stack>
        </Stack>
    }
}
