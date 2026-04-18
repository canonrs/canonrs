use leptos::prelude::*;
use super::breadcrumb_boundary::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbPage, BreadcrumbSeparator, BreadcrumbEllipsis,
};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn BreadcrumbShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
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
            <p data-rs-showcase-preview-anchor="">
                "Current page state enforced via activity state mapping."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Custom separator"</span>
                <Breadcrumb>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator>"›"</BreadcrumbSeparator>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="#">"Settings"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator>"›"</BreadcrumbSeparator>
                    <BreadcrumbItem>
                        <BreadcrumbPage>"Profile"</BreadcrumbPage>
                    </BreadcrumbItem>
                </Breadcrumb>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"With ellipsis"</span>
                <Breadcrumb>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                    <BreadcrumbItem>
                        <BreadcrumbEllipsis />
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
            </Stack>
        </Stack>
    }
}
