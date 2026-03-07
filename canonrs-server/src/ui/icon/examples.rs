use leptos::prelude::*;
use super::{Icon, IconSize, IconVariant};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            // Sizes
            <div>
                <h4>"Sizes"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Icon size=IconSize::Sm>"⭐"</Icon>
                    <Icon size=IconSize::Md>"⭐"</Icon>
                    <Icon size=IconSize::Lg>"⭐"</Icon>
                </div>
            </div>

            // Variants
            <div>
                <h4>"Variants"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Icon variant=IconVariant::Default>"🔵"</Icon>
                    <Icon variant=IconVariant::Muted>"🔘"</Icon>
                    <Icon variant=IconVariant::Primary>"💙"</Icon>
                    <Icon variant=IconVariant::Destructive>"❌"</Icon>
                    <Icon variant=IconVariant::Success>"✅"</Icon>
                    <Icon variant=IconVariant::Warning>"⚠️"</Icon>
                </div>
            </div>

            // Spin
            <div>
                <h4>"Spin (Loading)"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Icon spin=true>"⏳"</Icon>
                    <Icon size=IconSize::Lg spin=true>"🔄"</Icon>
                </div>
            </div>
        </div>
    }
}
