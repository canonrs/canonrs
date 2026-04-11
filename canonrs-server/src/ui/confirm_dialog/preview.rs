use leptos::prelude::*;
use super::confirm_dialog_island::{
    ConfirmDialogIsland, ConfirmDialogTriggerIsland, ConfirmDialogOverlayIsland,
    ConfirmDialogContentIsland, ConfirmDialogTitleIsland, ConfirmDialogDescriptionIsland,
    ConfirmDialogFooterIsland, ConfirmDialogCancelIsland, ConfirmDialogConfirmIsland,
};
use canonrs_core::primitives::ConfirmDialogVariant;

#[component]
pub fn ConfirmDialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ConfirmDialogIsland>
                    <ConfirmDialogTriggerIsland>"Delete item"</ConfirmDialogTriggerIsland>
                    <ConfirmDialogOverlayIsland />
                    <ConfirmDialogContentIsland>
                        <ConfirmDialogTitleIsland>"Are you sure?"</ConfirmDialogTitleIsland>
                        <ConfirmDialogDescriptionIsland>"This action cannot be undone."</ConfirmDialogDescriptionIsland>
                        <ConfirmDialogFooterIsland>
                            <ConfirmDialogCancelIsland>"Cancel"</ConfirmDialogCancelIsland>
                            <ConfirmDialogConfirmIsland variant=ConfirmDialogVariant::Destructive>"Delete"</ConfirmDialogConfirmIsland>
                        </ConfirmDialogFooterIsland>
                    </ConfirmDialogContentIsland>
                </ConfirmDialogIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Confirm dialog enforces action confirmation with variant-aware actions."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Warning variant"</span>
                <div data-rs-showcase-preview-row="">
                    <ConfirmDialogIsland variant=ConfirmDialogVariant::Warning>
                        <ConfirmDialogTriggerIsland variant=ConfirmDialogVariant::Warning>"Archive project"</ConfirmDialogTriggerIsland>
                        <ConfirmDialogOverlayIsland />
                        <ConfirmDialogContentIsland>
                            <ConfirmDialogTitleIsland>"Archive this project?"</ConfirmDialogTitleIsland>
                            <ConfirmDialogDescriptionIsland>"The project will be archived and hidden from your dashboard."</ConfirmDialogDescriptionIsland>
                            <ConfirmDialogFooterIsland>
                                <ConfirmDialogCancelIsland>"Cancel"</ConfirmDialogCancelIsland>
                                <ConfirmDialogConfirmIsland variant=ConfirmDialogVariant::Warning>"Archive"</ConfirmDialogConfirmIsland>
                            </ConfirmDialogFooterIsland>
                        </ConfirmDialogContentIsland>
                    </ConfirmDialogIsland>
                </div>
            </div>
        </div>
    }
}
