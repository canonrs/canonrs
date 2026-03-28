//! @canon-id: link-group
//! @canon-label: Link Group
//! @canon-family: navigation
//! @canon-category: Navigation
//! @canon-intent: Semantic group of navigation links with optional label
//! @canon-description: Wrapper that organizes multiple NavItems into a labeled navigation group
//! @canon-composable: true
//! @canon-capabilities: Orientation, Disabled
//! @canon-required-parts:
//! @canon-optional-parts:
//! @canon-tags: link-group, nav-group, navigation, links, footer, sidebar, grouped

use leptos::prelude::*;
use canonrs_core::primitives::{NavigationGroupPrimitive, NavigationGroupLabelPrimitive};

#[derive(Clone, Copy, PartialEq, Default)]
pub enum LinkGroupDirection {
    #[default]
    Vertical,
    Horizontal,
}

impl LinkGroupDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical   => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[component]
pub fn LinkGroup(
    children: Children,
    #[prop(optional)] label: Option<ChildrenFn>,
    #[prop(default = LinkGroupDirection::Vertical)] direction: LinkGroupDirection,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavigationGroupPrimitive class=class>
            {label.map(|l| view! {
                <NavigationGroupLabelPrimitive>
                    {l()}
                </NavigationGroupLabelPrimitive>
            })}
            <div data-rs-link-group-items="" data-rs-direction=direction.as_str()>
                {children()}
            </div>
        </NavigationGroupPrimitive>
    }
}

#[component]
pub fn LinkGroupPreview() -> impl IntoView {
    use super::super::nav_item::NavItem;
    view! {
        <LinkGroup label=std::sync::Arc::new(|| view! { "Product" }.into_any())>
            <NavItem label="Features".to_string() href="/features".to_string() />
            <NavItem label="Pricing".to_string()  href="/pricing".to_string() />
        </LinkGroup>
    }
}
