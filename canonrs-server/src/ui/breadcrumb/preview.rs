use leptos::prelude::*;
use super::breadcrumb_island::{BreadcrumbIsland, BreadcrumbIslandItem};

#[component]
pub fn BreadcrumbShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <BreadcrumbIsland items=vec![
                    BreadcrumbIslandItem { label: "Home".into(),       href: Some("#".into()), is_page: false },
                    BreadcrumbIslandItem { label: "Components".into(), href: Some("#".into()), is_page: false },
                    BreadcrumbIslandItem { label: "Breadcrumb".into(), href: None,             is_page: true  },
                ] />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Current page state enforced via activity state mapping."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Custom separator"</span>
                <div data-rs-showcase-preview-row="">
                    <BreadcrumbIsland
                        separator="›"
                        items=vec![
                            BreadcrumbIslandItem { label: "Home".into(),    href: Some("#".into()), is_page: false },
                            BreadcrumbIslandItem { label: "Current".into(), href: None,             is_page: true  },
                        ]
                    />
                </div>
            </div>
        </div>
    }
}
