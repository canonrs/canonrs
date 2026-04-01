use leptos::prelude::*;
use super::dialog_ui::{
    Dialog, DialogTrigger, DialogPortal, DialogOverlay,
    DialogContent, DialogTitle, DialogDescription, DialogClose,
};

#[component]
pub fn DialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Dialog>
                    <DialogTrigger>"Open Dialog"</DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay />
                        <DialogContent>
                            <DialogTitle>"Confirm action"</DialogTitle>
                            <DialogDescription>"Are you sure you want to proceed? This action cannot be undone."</DialogDescription>
                            <DialogClose>"Cancel"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Dialog accessibility and lifecycle enforced via primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Form dialog"</span>
                <div data-rs-showcase-preview-row="">
                    <Dialog>
                        <DialogTrigger>"Edit profile"</DialogTrigger>
                        <DialogPortal>
                            <DialogOverlay />
                            <DialogContent>
                                <DialogTitle>"Edit profile"</DialogTitle>
                                <DialogDescription>"Update your profile information below."</DialogDescription>
                                <DialogClose>"Save changes"</DialogClose>
                            </DialogContent>
                        </DialogPortal>
                    </Dialog>
                </div>
            </div>
        </div>
    }
}
