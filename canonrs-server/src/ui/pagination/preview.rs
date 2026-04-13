use leptos::prelude::*;
use super::pagination_boundary::{
    Pagination, PaginationContent, PaginationItem,
    PaginationLink, PaginationPrevious, PaginationNext,
};
use canonrs_core::meta::{ActivityState, DisabledState};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn PaginationShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Navigation state and accessibility governed by DOM."
            </p>
            <Pagination>
                <PaginationContent>
                    <PaginationItem>
                        <PaginationPrevious href="#">"←"</PaginationPrevious>
                    </PaginationItem>
                    <PaginationItem>
                        <PaginationLink href="#" state=ActivityState::Active>"1"</PaginationLink>
                    </PaginationItem>
                    <PaginationItem>
                        <PaginationLink href="#">"2"</PaginationLink>
                    </PaginationItem>
                    <PaginationItem>
                        <PaginationLink href="#">"3"</PaginationLink>
                    </PaginationItem>
                    <PaginationItem>
                        <PaginationNext href="#">"→"</PaginationNext>
                    </PaginationItem>
                </PaginationContent>
            </Pagination>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"First page (prev disabled)"</span>
                <Pagination>
                    <PaginationContent>
                        <PaginationItem>
                            <PaginationPrevious href="#" disabled=DisabledState::Disabled>"←"</PaginationPrevious>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationLink href="#" state=ActivityState::Active>"1"</PaginationLink>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationNext href="#">"→"</PaginationNext>
                        </PaginationItem>
                    </PaginationContent>
                </Pagination>
            </Stack>
        </Stack>
    }
}
