use leptos::prelude::*;
use super::pagination_island::{
    PaginationIsland, PaginationContentIsland, PaginationItemIsland,
    PaginationLinkIsland, PaginationPreviousIsland, PaginationNextIsland,
};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn PaginationShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <PaginationIsland>
                    <PaginationContentIsland>
                        <PaginationItemIsland>
                            <PaginationPreviousIsland href="#">"←"</PaginationPreviousIsland>
                        </PaginationItemIsland>
                        <PaginationItemIsland>
                            <PaginationLinkIsland href="#" state=ActivityState::Active>"1"</PaginationLinkIsland>
                        </PaginationItemIsland>
                        <PaginationItemIsland>
                            <PaginationLinkIsland href="#">"2"</PaginationLinkIsland>
                        </PaginationItemIsland>
                        <PaginationItemIsland>
                            <PaginationLinkIsland href="#">"3"</PaginationLinkIsland>
                        </PaginationItemIsland>
                        <PaginationItemIsland>
                            <PaginationNextIsland href="#">"→"</PaginationNextIsland>
                        </PaginationItemIsland>
                    </PaginationContentIsland>
                </PaginationIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation state and accessibility governed by DOM."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"First page (prev disabled)"</span>
                <div data-rs-showcase-preview-row="">
                    <PaginationIsland>
                        <PaginationContentIsland>
                            <PaginationItemIsland>
                                <PaginationPreviousIsland href="#" disabled=DisabledState::Disabled>"←"</PaginationPreviousIsland>
                            </PaginationItemIsland>
                            <PaginationItemIsland>
                                <PaginationLinkIsland href="#" state=ActivityState::Active>"1"</PaginationLinkIsland>
                            </PaginationItemIsland>
                            <PaginationItemIsland>
                                <PaginationNextIsland href="#">"→"</PaginationNextIsland>
                            </PaginationItemIsland>
                        </PaginationContentIsland>
                    </PaginationIsland>
                </div>
            </div>
        </div>
    }
}
