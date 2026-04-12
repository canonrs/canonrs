use leptos::prelude::*;
use super::boundary::{
    ConfirmDialog, ConfirmDialogTrigger, ConfirmDialogOverlay,
    ConfirmDialogContent, ConfirmDialogTitle, ConfirmDialogDescription,
    ConfirmDialogFooter, ConfirmDialogCancel, ConfirmDialogConfirm,
};
use canonrs_core::primitives::ConfirmDialogVariant;

#[component]
pub fn ConfirmDialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ConfirmDialog>
                    <ConfirmDialogTrigger>"Delete item"</ConfirmDialogTrigger>
                    <ConfirmDialogOverlay />
                    <ConfirmDialogContent>
                        <ConfirmDialogTitle>"Are you sure?"</ConfirmDialogTitle>
                        <ConfirmDialogDescription>"This action cannot be undone."</ConfirmDialogDescription>
                        <ConfirmDialogFooter>
                            <ConfirmDialogCancel>"Cancel"</ConfirmDialogCancel>
                            <ConfirmDialogConfirm variant=ConfirmDialogVariant::Destructive>"Delete"</ConfirmDialogConfirm>
                        </ConfirmDialogFooter>
                    </ConfirmDialogContent>
                </ConfirmDialog>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Confirm dialog enforces action confirmation with variant-aware actions."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Warning variant"</span>
                <div data-rs-showcase-preview-row="">
                    <ConfirmDialog variant=ConfirmDialogVariant::Warning>
                        <ConfirmDialogTrigger variant=ConfirmDialogVariant::Warning>"Archive project"</ConfirmDialogTrigger>
                        <ConfirmDialogOverlay />
                        <ConfirmDialogContent>
                            <ConfirmDialogTitle>"Archive this project?"</ConfirmDialogTitle>
                            <ConfirmDialogDescription>"The project will be archived and hidden from your dashboard."</ConfirmDialogDescription>
                            <ConfirmDialogFooter>
                                <ConfirmDialogCancel>"Cancel"</ConfirmDialogCancel>
                                <ConfirmDialogConfirm variant=ConfirmDialogVariant::Warning>"Archive"</ConfirmDialogConfirm>
                            </ConfirmDialogFooter>
                        </ConfirmDialogContent>
                    </ConfirmDialog>
                </div>
            </div>
        </div>
    }
}
