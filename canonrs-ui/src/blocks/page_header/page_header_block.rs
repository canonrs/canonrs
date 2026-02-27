//! # PageHeader Block

use leptos::prelude::*;

#[component]
pub fn PageHeader(
    #[prop(into)] title: String,
    #[prop(into, default = String::new())] subtitle: String,
    #[prop(optional)] breadcrumb: Option<ChildrenFn>,
    #[prop(optional)] actions: Option<ChildrenFn>,
    #[prop(default = String::new(), into)] class: String,
    #[prop(default = String::new(), into)] id: String,
) -> impl IntoView {
    view! {
        <div
            class=format!("page-header {}", class)
            id=if id.is_empty() { None } else { Some(id) }
            data-block="page-header"
            data-block-version="1"
        >
            <div data-block-region="breadcrumb">{breadcrumb.map(|bc| bc())}</div>
            <div data-block-region="content">
                <div data-block-region="text">
                    <h1 class="page-header__title">{title}</h1>
                    {(!subtitle.is_empty()).then(|| view! {
                        <p class="page-header__subtitle">{subtitle}</p>
                    })}
                </div>
                <div data-block-region="actions">{actions.map(|a| a())}</div>
            </div>
        </div>
    }
}
