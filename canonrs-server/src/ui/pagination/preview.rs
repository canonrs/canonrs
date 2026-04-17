use leptos::prelude::*;
use super::pagination_boundary::{
    Pagination, PaginationContent, PaginationItem,
    PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis,
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
            <Pagination current_page=3 total_pages=10>
                <PaginationContent>
                    <PaginationItem>
                        <PaginationPrevious href="#">"←"</PaginationPrevious>
                    </PaginationItem>
                    <PaginationItem><PaginationLink href="#" page=1>"1"</PaginationLink></PaginationItem>
                    <PaginationItem><PaginationLink href="#" page=2>"2"</PaginationLink></PaginationItem>
                    <PaginationItem><PaginationLink href="#" page=3 state=ActivityState::Active>"3"</PaginationLink></PaginationItem>
                    <PaginationItem><PaginationLink href="#" page=4>"4"</PaginationLink></PaginationItem>
                    <PaginationItem><PaginationEllipsis /></PaginationItem>
                    <PaginationItem><PaginationLink href="#" page=10>"10"</PaginationLink></PaginationItem>
                    <PaginationItem>
                        <PaginationNext href="#">"→"</PaginationNext>
                    </PaginationItem>
                </PaginationContent>
            </Pagination>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"First page (prev disabled)"</span>
                <Pagination current_page=1 total_pages=5>
                    <PaginationContent>
                        <PaginationItem>
                            <PaginationPrevious href="#" disabled=DisabledState::Disabled>"←"</PaginationPrevious>
                        </PaginationItem>
                        <PaginationItem><PaginationLink href="#" page=1 state=ActivityState::Active>"1"</PaginationLink></PaginationItem>
                        <PaginationItem><PaginationLink href="#" page=2>"2"</PaginationLink></PaginationItem>
                        <PaginationItem><PaginationLink href="#" page=3>"3"</PaginationLink></PaginationItem>
                        <PaginationItem>
                            <PaginationNext href="#">"→"</PaginationNext>
                        </PaginationItem>
                    </PaginationContent>
                </Pagination>
            </Stack>
        </Stack>
    }
}
