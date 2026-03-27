use leptos::prelude::*;
use super::{Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};
use canonrs_core::primitives::{BreadcrumbPrimitive, BreadcrumbItemPrimitive, BreadcrumbLinkPrimitive, BreadcrumbSeparatorPrimitive};
use canonrs_core::meta::ActivityState;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            <div>
                <h4>"Basic Breadcrumb"</h4>
                <Breadcrumb>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="/products">"Products"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <BreadcrumbLink href="/products/category">"Category"</BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <BreadcrumbPage>"Item"</BreadcrumbPage>
                    </BreadcrumbItem>
                </Breadcrumb>
            </div>

            <div>
                <h4>"Collapsible Breadcrumb"</h4>
                <BreadcrumbPrimitive class=String::new()>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbLinkPrimitive href="/".to_string() state=ActivityState::Inactive class=String::new()>"Home"</BreadcrumbLinkPrimitive>
                    </BreadcrumbItemPrimitive>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbSeparatorPrimitive class=String::new()>"/"</BreadcrumbSeparatorPrimitive>
                    </BreadcrumbItemPrimitive>

                    <details data-breadcrumb-collapse="" id="breadcrumb-collapse-1">
                        <summary data-breadcrumb-ellipsis-trigger="">"..."</summary>
                        <div data-breadcrumb-collapse-content="">
                            <BreadcrumbLinkPrimitive href="/docs".to_string() state=ActivityState::Inactive class=String::new()>"Documentation"</BreadcrumbLinkPrimitive>
                            <BreadcrumbLinkPrimitive href="/docs/components".to_string() state=ActivityState::Inactive class=String::new()>"Components"</BreadcrumbLinkPrimitive>
                            <BreadcrumbLinkPrimitive href="/docs/components/forms".to_string() state=ActivityState::Inactive class=String::new()>"Forms"</BreadcrumbLinkPrimitive>
                        </div>
                    </details>

                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbSeparatorPrimitive class=String::new()>"/"</BreadcrumbSeparatorPrimitive>
                    </BreadcrumbItemPrimitive>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbLinkPrimitive href="/products".to_string() state=ActivityState::Inactive class=String::new()>"Products"</BreadcrumbLinkPrimitive>
                    </BreadcrumbItemPrimitive>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <BreadcrumbSeparatorPrimitive class=String::new()>"/"</BreadcrumbSeparatorPrimitive>
                    </BreadcrumbItemPrimitive>
                    <BreadcrumbItemPrimitive class=String::new()>
                        <span data-rs-breadcrumb-page="" aria-current="page">"Current Item"</span>
                    </BreadcrumbItemPrimitive>
                </BreadcrumbPrimitive>
            </div>
        </div>
    }
}

#[component]
pub fn AutoExample() -> impl IntoView {
    use super::BreadcrumbAuto;
    use canonrs_core::{NavigationState, HeadingHierarchy, HeadingNode};
    use std::collections::HashMap;

    let mock_state = {
        let mut state = NavigationState::new();
        let headings = vec![
            HeadingNode {
                id: "introduction".to_string(),
                text: "Introduction".to_string(),
                level: 1,
                parent_id: None,
                children_ids: vec!["getting-started".to_string()],
            },
            HeadingNode {
                id: "getting-started".to_string(),
                text: "Getting Started".to_string(),
                level: 2,
                parent_id: Some("introduction".to_string()),
                children_ids: vec!["installation".to_string()],
            },
            HeadingNode {
                id: "installation".to_string(),
                text: "Installation".to_string(),
                level: 3,
                parent_id: Some("getting-started".to_string()),
                children_ids: vec![],
            },
        ];
        let mut id_to_index = HashMap::new();
        for (idx, node) in headings.iter().enumerate() {
            id_to_index.insert(node.id.clone(), idx);
        }
        state.heading_hierarchy = HeadingHierarchy { headings, id_to_index };
        state.current_heading_id = Some("installation".to_string());
        state
    };

    let state_signal = RwSignal::new(mock_state);
    provide_context(state_signal);

    view! {
        <div>
            <h4>"Auto Breadcrumb (from NavigationContext)"</h4>
            <BreadcrumbAuto />
        </div>
    }
}
