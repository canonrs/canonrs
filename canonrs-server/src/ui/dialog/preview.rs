use leptos::prelude::*;
use super::dialog_island::{
    DialogIsland, DialogTriggerIsland, DialogPortalIsland,
    DialogOverlayIsland, DialogContentIsland, DialogTitleIsland,
    DialogDescriptionIsland, DialogCloseIsland,
};

#[component]
pub fn DialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <DialogIsland>
                    <DialogTriggerIsland>"Open Dialog"</DialogTriggerIsland>
                    <DialogPortalIsland>
                        <DialogOverlayIsland />
                        <DialogContentIsland>
                            <DialogTitleIsland>"Confirm action"</DialogTitleIsland>
                            <DialogDescriptionIsland>"Are you sure? This action cannot be undone."</DialogDescriptionIsland>
                            <DialogCloseIsland>"Cancel"</DialogCloseIsland>
                        </DialogContentIsland>
                    </DialogPortalIsland>
                </DialogIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Dialog accessibility and lifecycle enforced via primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Form dialog"</span>
                <div data-rs-showcase-preview-row="">
                    <DialogIsland>
                        <DialogTriggerIsland>"Edit profile"</DialogTriggerIsland>
                        <DialogPortalIsland>
                            <DialogOverlayIsland />
                            <DialogContentIsland>
                                <DialogTitleIsland>"Edit profile"</DialogTitleIsland>
                                <DialogDescriptionIsland>"Update your profile information below."</DialogDescriptionIsland>
                                <DialogCloseIsland>"Save changes"</DialogCloseIsland>
                            </DialogContentIsland>
                        </DialogPortalIsland>
                    </DialogIsland>
                </div>
            </div>
        </div>
    }
}
