use leptos::prelude::*;
use super::breadcrumb_ui::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbSeparator, BreadcrumbPage, BreadcrumbEllipsis,
};
use canonrs_core::meta::ActivityState;

#[component]
pub fn BreadcrumbShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Breadcrumb>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="#">"Components"</BreadcrumbLink>
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
                <span data-rs-showcase-preview-label="">"With ellipsis"</span>
                <div data-rs-showcase-preview-row="">
                    <Breadcrumb>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                        <BreadcrumbEllipsis />
                        <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="#">"Components"</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                        <BreadcrumbItem>
                            <BreadcrumbPage>"Breadcrumb"</BreadcrumbPage>
                        </BreadcrumbItem>
                    </Breadcrumb>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Active link"</span>
                <div data-rs-showcase-preview-row="">
                    <Breadcrumb>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="#" state=ActivityState::Active>"Home"</BreadcrumbLink>
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
