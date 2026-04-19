use leptos::prelude::*;
use super::confirm_dialog_boundary::{
    ConfirmDialog, ConfirmDialogTrigger, ConfirmDialogPortal, ConfirmDialogOverlay,
    ConfirmDialogContent, ConfirmDialogTitle, ConfirmDialogDescription,
    ConfirmDialogFooter, ConfirmDialogCancel, ConfirmDialogConfirm,
};
use canonrs_core::primitives::ConfirmDialogVariant;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ConfirmDialogShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <ConfirmDialog>
                <ConfirmDialogTrigger>"Delete item"</ConfirmDialogTrigger>
                <ConfirmDialogPortal>
                    <ConfirmDialogOverlay />
                    <ConfirmDialogContent>
                        <ConfirmDialogTitle>"Are you sure?"</ConfirmDialogTitle>
                        <ConfirmDialogDescription>"This action cannot be undone."</ConfirmDialogDescription>
                        <ConfirmDialogFooter>
                            <ConfirmDialogCancel>"Cancel"</ConfirmDialogCancel>
                            <ConfirmDialogConfirm variant=ConfirmDialogVariant::Destructive>"Delete"</ConfirmDialogConfirm>
                        </ConfirmDialogFooter>
                    </ConfirmDialogContent>
                </ConfirmDialogPortal>
            </ConfirmDialog>
            <p data-rs-showcase-preview-anchor="">
                "Confirm dialog enforces action confirmation with variant-aware actions."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Warning variant"</span>
                <ConfirmDialog variant=ConfirmDialogVariant::Warning>
                    <ConfirmDialogTrigger variant=ConfirmDialogVariant::Warning>"Archive project"</ConfirmDialogTrigger>
                    <ConfirmDialogPortal>
                        <ConfirmDialogOverlay />
                        <ConfirmDialogContent>
                            <ConfirmDialogTitle>"Archive this project?"</ConfirmDialogTitle>
                            <ConfirmDialogDescription>"The project will be archived and hidden from your dashboard."</ConfirmDialogDescription>
                            <ConfirmDialogFooter>
                                <ConfirmDialogCancel>"Cancel"</ConfirmDialogCancel>
                                <ConfirmDialogConfirm variant=ConfirmDialogVariant::Warning>"Archive"</ConfirmDialogConfirm>
                            </ConfirmDialogFooter>
                        </ConfirmDialogContent>
                    </ConfirmDialogPortal>
                </ConfirmDialog>
            </Stack>
        </Stack>
    }
}
