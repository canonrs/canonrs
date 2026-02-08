use leptos::prelude::*;
use super::pagination_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationContent>
                <PaginationPrevious>"Previous"</PaginationPrevious>
                <PaginationItem>
                    <PaginationLink is_active=false>"1"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink is_active=true>"2"</PaginationLink>
                </PaginationItem>
                <PaginationNext>"Next"</PaginationNext>
            </PaginationContent>
        </Pagination>
    }
}
