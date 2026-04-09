use leptos::prelude::*;
use super::confirm_dialog_island::{
    ConfirmDialogIsland, ConfirmDialogTriggerIsland, ConfirmDialogPortalIsland,
    ConfirmDialogOverlayIsland, ConfirmDialogContentIsland, ConfirmDialogTitleIsland,
    ConfirmDialogDescriptionIsland, ConfirmDialogFooterIsland,
    ConfirmDialogCancelIsland, ConfirmDialogConfirmIsland,
};
use canonrs_core::primitives::ConfirmDialogVariant;

#[component]
pub fn ConfirmDialogShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ConfirmDialogIsland variant=ConfirmDialogVariant::Destructive>
                    <ConfirmDialogTriggerIsland variant=ConfirmDialogVariant::Destructive>"Delete item"</ConfirmDialogTriggerIsland>
                    <ConfirmDialogPortalIsland>
                        <ConfirmDialogOverlayIsland />
                        <ConfirmDialogContentIsland>
                            <ConfirmDialogTitleIsland>"Delete item"</ConfirmDialogTitleIsland>
                            <ConfirmDialogDescriptionIsland>"Are you sure? This action cannot be undone."</ConfirmDialogDescriptionIsland>
                            <ConfirmDialogFooterIsland>
                                <ConfirmDialogCancelIsland>"Cancel"</ConfirmDialogCancelIsland>
                                <ConfirmDialogConfirmIsland variant=ConfirmDialogVariant::Destructive>"Delete"</ConfirmDialogConfirmIsland>
                            </ConfirmDialogFooterIsland>
                        </ConfirmDialogContentIsland>
                    </ConfirmDialogPortalIsland>
                </ConfirmDialogIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Confirmation semantics and intent enforced via variant and structure."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Non-destructive"</span>
                <div data-rs-showcase-preview-row="">
                    <ConfirmDialogIsland>
                        <ConfirmDialogTriggerIsland variant=ConfirmDialogVariant::Default>"Save changes"</ConfirmDialogTriggerIsland>
                        <ConfirmDialogPortalIsland>
                            <ConfirmDialogOverlayIsland />
                            <ConfirmDialogContentIsland>
                                <ConfirmDialogTitleIsland>"Save changes"</ConfirmDialogTitleIsland>
                                <ConfirmDialogDescriptionIsland>"Do you want to save your changes before leaving?"</ConfirmDialogDescriptionIsland>
                                <ConfirmDialogFooterIsland>
                                    <ConfirmDialogCancelIsland>"Discard"</ConfirmDialogCancelIsland>
                                    <ConfirmDialogConfirmIsland>"Save"</ConfirmDialogConfirmIsland>
                                </ConfirmDialogFooterIsland>
                            </ConfirmDialogContentIsland>
                        </ConfirmDialogPortalIsland>
                    </ConfirmDialogIsland>
                </div>
            </div>
        </div>
    }
}
