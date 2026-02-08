use leptos::prelude::*;
use super::{Breadcrumb, BreadcrumbItemData, BreadcrumbEllipsis};

#[component]
pub fn basic_example() -> impl IntoView {
    let items = vec![
        BreadcrumbItemData {
            label: "Home".to_string(),
            href: "/".to_string(),
            is_current: false,
        },
        BreadcrumbItemData {
            label: "Products".to_string(),
            href: "/products".to_string(),
            is_current: false,
        },
        BreadcrumbItemData {
            label: "Category".to_string(),
            href: "/products/category".to_string(),
            is_current: false,
        },
        BreadcrumbItemData {
            label: "Item".to_string(),
            href: "/products/category/item".to_string(),
            is_current: true,
        },
    ];

    view! {
        <Breadcrumb items=items />
    }
}

#[component]
pub fn custom_separator_example() -> impl IntoView {
    let items = vec![
        BreadcrumbItemData {
            label: "Home".to_string(),
            href: "/".to_string(),
            is_current: false,
        },
        BreadcrumbItemData {
            label: "Docs".to_string(),
            href: "/docs".to_string(),
            is_current: false,
        },
        BreadcrumbItemData {
            label: "Components".to_string(),
            href: "/docs/components".to_string(),
            is_current: true,
        },
    ];

    view! {
        <Breadcrumb items=items separator="→".to_string() />
    }
}

#[component]
pub fn with_ellipsis_example() -> impl IntoView {
    let items = vec![
        BreadcrumbItemData {
            label: "Home".to_string(),
            href: "/".to_string(),
            is_current: false,
        },
        BreadcrumbItemData {
            label: "Current".to_string(),
            href: "/current".to_string(),
            is_current: true,
        },
    ];

    view! {
        <div style="display: flex; align-items: center; gap: 0.5rem;">
            <Breadcrumb items=items />
            <BreadcrumbEllipsis />
        </div>
    }
}
