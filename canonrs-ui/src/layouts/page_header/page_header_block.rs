//! PageHeader Block
//! Estrutura padrão de cabeçalho de página enterprise

use leptos::prelude::*;

#[component]
pub fn PageHeader(
    #[prop(into)] title: String,
    #[prop(into, default = String::new())] subtitle: String,
    #[prop(optional)] breadcrumb: Option<Children>,
    #[prop(optional)] actions: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            class={format!("page-header {}", class)}
            id={if id.is_empty() { None } else { Some(id.clone()) }}
            data-layout="page-header"
        >
            {breadcrumb.map(|bc| view! {
                <div class="page-header__breadcrumb">
                    {bc()}
                </div>
            })}

            <div class="page-header__content">
                <div class="page-header__text">
                    <h1 class="page-header__title">{title}</h1>
                    {(!subtitle.is_empty()).then(|| view! {
                        <p class="page-header__subtitle">{subtitle}</p>
                    })}
                </div>

                {actions.map(|acts| view! {
                    <div class="page-header__actions">
                        {acts()}
                    </div>
                })}
            </div>
        </div>
    }
}
