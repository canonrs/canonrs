use leptos::prelude::*;
use super::breadcrumb_boundary::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbPage, BreadcrumbSeparator,
};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn BreadcrumbShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
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
            <p data-rs-showcase-preview-anchor="">
                "Current page state enforced via activity state mapping."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Custom separator"</span>
                <Breadcrumb>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator>"›"</BreadcrumbSeparator>
                    <BreadcrumbItem>
                        <BreadcrumbPage>"Current"</BreadcrumbPage>
                    </BreadcrumbItem>
                </Breadcrumb>
            </Stack>
        </Stack>
    }
}
