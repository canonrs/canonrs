use leptos::prelude::*;
use super::pagination_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious href="/page/1">"← Previous"</PaginationPrevious>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="/page/1">"1"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="/page/2" is_active=true>"2"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="/page/3">"3"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext href="/page/3">"Next →"</PaginationNext>
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}

pub fn with_ellipsis_example() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious href="/page/4">"← Previous"</PaginationPrevious>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="/page/1">"1"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="/page/5" is_active=true>"5"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationEllipsis />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="/page/10">"10"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext href="/page/6">"Next →"</PaginationNext>
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}

pub fn disabled_example() -> impl IntoView {
    view! {
        <Pagination>
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious disabled=true>"← Previous"</PaginationPrevious>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="/page/1" is_active=true>"1"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink href="/page/2">"2"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext href="/page/2">"Next →"</PaginationNext>
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
