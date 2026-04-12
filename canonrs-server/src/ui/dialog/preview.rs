use leptos::prelude::*;
use super::dialog_boundary::{
    Dialog, DialogTrigger, DialogOverlay,
    DialogContent, DialogTitle, DialogDescription,
    DialogClose, DialogFooter,
};
use crate::ui::button::Button;
use canonrs_core::primitives::ButtonVariant;

#[component]
pub fn DialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Dialog>
                    <DialogTrigger>"Open Dialog"</DialogTrigger>
                    <DialogOverlay />
                    <DialogContent>
                        <DialogTitle>"Confirm action"</DialogTitle>
                        <DialogDescription>"Are you sure? This action cannot be undone."</DialogDescription>
                        <DialogFooter>
                            <DialogClose>"Cancel"</DialogClose>
                            <Button variant=ButtonVariant::Primary>"Confirm"</Button>
                        </DialogFooter>
                    </DialogContent>
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
                        <DialogOverlay />
                        <DialogContent>
                            <DialogTitle>"Edit profile"</DialogTitle>
                            <DialogDescription>"Update your profile information below."</DialogDescription>
                            <DialogFooter>
                                <DialogClose>"Cancel"</DialogClose>
                                <Button variant=ButtonVariant::Primary>"Save changes"</Button>
                            </DialogFooter>
                        </DialogContent>
                    </Dialog>
                </div>
            </div>
        </div>
    }
}
