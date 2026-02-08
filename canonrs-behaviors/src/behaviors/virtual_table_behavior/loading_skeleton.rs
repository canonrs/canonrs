use leptos::prelude::*;

#[component]
pub fn LoadingSkeleton(
    #[prop(default = 10)] rows: usize,
    #[prop(default = 36.0)] row_height: f64,
) -> impl IntoView {
    view! {
        <div data-loading-skeleton="">
            {(0..rows).map(|_| view! {
                <div
                    attr:data-skeleton-row=""
                    style={format!("--row-height: {}px;", row_height)}
                >
                    <div data-skeleton-cell="" data-flex="1"></div>
                    <div data-skeleton-cell="" data-width="6rem"></div>
                    <div data-skeleton-cell="" data-flex="2"></div>
                </div>
            }).collect_view()}
        </div>
    }
}
