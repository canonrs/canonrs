//! # Section Layout
//!
//! **Canon Rule:**
//! - Section é LAYOUT (define regiões)
//! - NÃO contém lógica
//! - NÃO depende de UI
//! - Expõe apenas slots semânticos

use leptos::prelude::*;

#[component]
pub fn Section(
    /// Conteúdo principal da section
    children: Children,

    /// Header opcional (título, ações, breadcrumbs)
    #[prop(optional)]
    header: Option<Children>,

    /// Footer opcional (ações secundárias)
    #[prop(optional)]
    footer: Option<Children>,

    /// Classe adicional
    #[prop(into, default = String::new())]
    class: String,

    /// ID opcional
    #[prop(into, default = String::new())]
    id: String,
) -> impl IntoView {
    view! {
        <section
            id={id}
            class={format!("layout-section {}", class)}
            data-layout="section"
        >
            {header.map(|h| view! {
                <div
                    class="layout-section__header"
                    data-layout-region="header"
                >
                    {h()}
                </div>
            })}

            <div
                class="layout-section__content"
                data-layout-region="content"
            >
                {children()}
            </div>

            {footer.map(|f| view! {
                <div
                    class="layout-section__footer"
                    data-layout-region="footer"
                >
                    {f()}
                </div>
            })}
        </section>
    }
}
