use leptos::prelude::*;
use super::pagination_ui::{
    Pagination, PaginationContent, PaginationItem,
    PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis,
};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn PaginationShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Pagination>
                    <PaginationContent>
                        <PaginationItem>
                            <PaginationPrevious href="#">"←"</PaginationPrevious>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationLink href="#">"1"</PaginationLink>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationLink href="#" state=ActivityState::Active>"2"</PaginationLink>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationLink href="#">"3"</PaginationLink>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationEllipsis />
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationLink href="#">"10"</PaginationLink>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationNext href="#">"→"</PaginationNext>
                        </PaginationItem>
                    </PaginationContent>
                </Pagination>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation state and accessibility enforced via structured primitives."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"First page (prev disabled)"</span>
                <div data-rs-showcase-preview-row="">
                    <Pagination>
                        <PaginationContent>
                            <PaginationItem>
                                <PaginationPrevious href="#" disabled=DisabledState::Disabled>"←"</PaginationPrevious>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#" state=ActivityState::Active>"1"</PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationLink href="#">"2"</PaginationLink>
                            </PaginationItem>
                            <PaginationItem>
                                <PaginationNext href="#">"→"</PaginationNext>
                            </PaginationItem>
                        </PaginationContent>
                    </Pagination>
                </div>
            </div>
        </div>
    }
}
