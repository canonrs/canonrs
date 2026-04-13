use leptos::prelude::*;
use super::dialog_boundary::{
    Dialog, DialogTrigger, DialogOverlay,
    DialogContent, DialogTitle, DialogDescription,
    DialogClose, DialogFooter,
};
use crate::ui::button::button_boundary::Button;
use canonrs_core::primitives::ButtonVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn DialogShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
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
            <p data-rs-showcase-preview-anchor="">
                "Dialog accessibility and lifecycle enforced via primitives."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Form dialog"</span>
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
            </Stack>
        </Stack>
    }
}
