use leptos::prelude::*;
use crate::primitives::{
    BreadcrumbPrimitive, BreadcrumbItemPrimitive, BreadcrumbLinkPrimitive,
    BreadcrumbSeparatorPrimitive, BreadcrumbEllipsisPrimitive,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbItemData {
    pub label: String,
    pub href: String,
    pub is_current: bool,
}

#[component]
pub fn Breadcrumb(
    items: Vec<BreadcrumbItemData>,
    #[prop(default = String::new())] separator: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    let sep = if separator.is_empty() { "/".to_string() } else { separator.clone() };
    let total = items.len();

    view! {
        <BreadcrumbPrimitive class={class} id={id}>
            {items.into_iter().enumerate().map(|(idx, item)| {
                let is_last = idx == total - 1;
                let label = item.label.clone();
                let href = item.href.clone();
                let sep_clone = sep.clone();

                view! {
                    <BreadcrumbItemPrimitive class=String::new()>
                        {if is_last {
                            view! { <span data-breadcrumb-page="" attr:aria-current="page">{label}</span> }.into_any()
                        } else {
                            view! { <BreadcrumbLinkPrimitive href=href current={false} class=String::new()>{label}</BreadcrumbLinkPrimitive> }.into_any()
                        }}
                    </BreadcrumbItemPrimitive>
                    {(!is_last).then(move || view! {
                        <BreadcrumbItemPrimitive class=String::new()>
                            <BreadcrumbSeparatorPrimitive class=String::new()>{sep_clone.clone()}</BreadcrumbSeparatorPrimitive>
                        </BreadcrumbItemPrimitive>
                    })}
                }
            }).collect_view()}
        </BreadcrumbPrimitive>
    }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] href: String,
    #[prop(default = false)] current: bool,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! { <BreadcrumbLinkPrimitive href={href} current={current} class={class}>{children.map(|c| c())}</BreadcrumbLinkPrimitive> }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! { <BreadcrumbSeparatorPrimitive class={class}>{children.map(|c| c())}</BreadcrumbSeparatorPrimitive> }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! { <BreadcrumbItemPrimitive class={String::new()}><BreadcrumbEllipsisPrimitive class={class} id={id} /></BreadcrumbItemPrimitive> }
}
