use leptos::prelude::*;
use super::boundary::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbPage, BreadcrumbSeparator,
};

#[component]
pub fn BreadcrumbShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Breadcrumb>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="/components">"Components"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                    <BreadcrumbItem>
                        <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                    </BreadcrumbItem>
                </Breadcrumb>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Current page state enforced via activity state mapping."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Custom separator"</span>
                <div data-rs-showcase-preview-row="">
                    <Breadcrumb>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator>"›"</BreadcrumbSeparator>
                        <BreadcrumbItem>
                            <BreadcrumbPage>"Current"</BreadcrumbPage>
                        </BreadcrumbItem>
                    </Breadcrumb>
                </div>
            </div>
        </div>
    }
}
