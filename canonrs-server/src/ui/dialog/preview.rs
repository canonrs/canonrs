use leptos::prelude::*;
use super::dialog_island::DialogIsland;

#[component]
pub fn DialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DialogIsland
                    trigger_label="Open Dialog"
                    title="Confirm action"
                    description="Are you sure? This action cannot be undone."
                    close_label="Cancel"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Dialog accessibility and lifecycle enforced via primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Form dialog"</span>
                <div data-rs-showcase-preview-row="">
                    <DialogIsland
                        trigger_label="Edit profile"
                        title="Edit profile"
                        description="Update your profile information below."
                        close_label="Save changes"
                    />
                </div>
            </div>
        </div>
    }
}
