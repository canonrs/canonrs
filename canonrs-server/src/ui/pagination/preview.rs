use leptos::prelude::*;
use super::pagination_island::PaginationIsland;

#[component]
pub fn PaginationShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <PaginationIsland total_pages=10 current_page=2 />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation state and accessibility governed by signal."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"First page (prev disabled)"</span>
                <div data-rs-showcase-preview-row="">
                    <PaginationIsland total_pages=5 current_page=1 />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Last page (next disabled)"</span>
                <div data-rs-showcase-preview-row="">
                    <PaginationIsland total_pages=5 current_page=5 />
                </div>
            </div>
        </div>
    }
}
